use hashbrown::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use super::blocks::AirBlock;
use super::block_state::{BlockState, blockstate};
use super::{block::BlockBehavior, id::{BlockId, StateId}};
use super::error::{Error, Result};

// main


/// `blocks` represents the list of [BlockBehavior]s (the trait that has functions for blocks).
/// 
/// `block_lookup` allows for looking up a [BlockBehavior]'s [BlockId], which points
/// back to the [BlockBehavior] in `blocks`.
/// 
/// `states` contains each [BlockState] contained in this registry.
/// 
/// `block_ids` contains the [BlockId]s associated with each state.
/// 
/// `state_lookup` is a lookup table of [BlockState]s that will return its [StateId].
struct InnerBlockRegistry {
    blocks: Vec<Arc<dyn BlockBehavior>>,
    block_lookup: HashMap<String, BlockId>,
    states: Vec<Arc<BlockState>>,
    block_ids: Vec<BlockId>,
    state_lookup: HashMap<Arc<BlockState>, StateId>,
}

impl Default for InnerBlockRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl InnerBlockRegistry {
    fn new() -> Self {
        let air_state = Arc::new(blockstate!(air));
        Self {
            blocks: vec![Arc::new(AirBlock)],
            block_lookup: HashMap::from([("air".to_owned(), BlockId(0))]),
            states: vec![air_state.clone()],
            block_ids: vec![BlockId(0)],
            state_lookup: HashMap::from([(air_state, StateId(0))]),
        }
    }
}

#[derive(Default, Clone)]
pub struct BlockRegistry(Arc<RwLock<InnerBlockRegistry>>);

impl BlockRegistry {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(InnerBlockRegistry::new())))
    }

    #[inline]
    fn read_lock<'a>(&'a self) -> Result<RwLockReadGuard<'a, InnerBlockRegistry>> {
        self.0.read().map_err(|_| Error::FailedToLock)
    }

    #[inline]
    fn write_lock<'a>(&'a self) -> Result<RwLockWriteGuard<'a, InnerBlockRegistry>> {
        self.0.write().map_err(|_| Error::FailedToLock)
    }

    pub fn register_block<B: BlockBehavior + 'static + Sized>(&self, block: B) -> Result<BlockId> {
        let mut reg = self.write_lock()?;
        if reg.blocks.len() > u32::MAX as usize {
            return Err(Error::RegistryOverflow);
        }
        if reg.block_lookup.contains_key(block.name()) {
            return Err(Error::DuplicateBlockEntry(block.name().to_owned()));
        }
        let block_id = BlockId(reg.blocks.len() as u32);
        let block = Arc::new(block);
        reg.blocks.push(block.clone());
        reg.block_lookup.insert(block.name().to_owned(), block_id);
        drop(reg);
        block.on_register(self);
        Ok(block_id)
    }

    pub fn register_state<S: RefOrOwned<BlockState>>(&self, state: S) -> Result<StateId> {
        let mut reg = self.write_lock()?;
        if let Some(&state_id) = reg.state_lookup.get(state.reference()) {
            Ok(state_id)
        } else {
            if reg.states.len() > u32::MAX as usize {
                return Err(Error::RegistryOverflow);
            }
            let state = state.owned();
            let Some(&block_id) = reg.block_lookup.get(state.name()) else {
                return Err(Error::BlockNotFound(state.name().to_owned()));
            };
            let state_id = StateId(reg.states.len() as u32);
            reg.block_ids.push(block_id);
            let state = Arc::new(state);
            reg.state_lookup.insert(state.clone(), state_id);
            reg.states.push(state);
            Ok(state_id)
        }
    }

    #[inline]
    pub fn get_block<I: BlockGetterId>(&self, id: I) -> Result<Arc<dyn BlockBehavior>> {
        id.get_block(self)
    }

    #[inline]
    pub fn access_block<I: BlockGetterId, R, F: FnOnce(Arc<dyn BlockBehavior>) -> R>(&self, id: I, access: F) -> Result<R> {
        let block = self.get_block(id)?;
        Ok(access(block))
    }

    #[inline]
    pub fn get_state(&self, id: StateId) -> Result<Arc<BlockState>> {
        let reg = self.read_lock()?;
        Ok(Arc::clone(&reg.states[id.index()]))
    }

    #[inline]
    pub fn access_state<R, F: FnOnce(Arc<BlockState>) -> R>(&self, id: StateId, access: F) -> Result<R> {
        let state = self.get_state(id)?;
        Ok(access(state))
    }

    #[inline]
    pub fn block_id(&self, id: StateId) -> Result<BlockId> {
        let reg = self.read_lock()?;
        let block_id = reg.block_ids[id.index()];
        Ok(block_id)
    }
}
mod sealed {
    pub trait BlockGetterSeal {}
    impl BlockGetterSeal for super::BlockId {}
    impl BlockGetterSeal for super::StateId {}
    impl<S: AsRef<str>> BlockGetterSeal for S {}
}

pub trait BlockGetterId: sealed::BlockGetterSeal {
    fn get_block(self, registry: &BlockRegistry) -> Result<Arc<dyn BlockBehavior>>;
}

impl BlockGetterId for BlockId {
    fn get_block(self, registry: &BlockRegistry) -> Result<Arc<dyn BlockBehavior>> {
        let reg = registry.read_lock()?;
        Ok(Arc::clone(&reg.blocks[self.index()]))
    }
}

impl BlockGetterId for StateId {
    fn get_block(self, registry: &BlockRegistry) -> Result<Arc<dyn BlockBehavior>> {
        let reg = registry.read_lock()?;
        let block_id = reg.block_ids[self.index()];
        Ok(Arc::clone(&reg.blocks[block_id.index()]))
    }
}

impl<S: AsRef<str>> BlockGetterId for S {
    fn get_block(self, registry: &BlockRegistry) -> Result<Arc<dyn BlockBehavior>> {
        let reg = registry.read_lock()?;
        if let Some(id) = reg.block_lookup.get(self.as_ref()) {
            Ok(Arc::clone(&reg.blocks[id.index()]))
        } else {
            Err(Error::BlockNotFound(self.as_ref().to_owned()))
        }
    }
}

pub trait RefOrOwned<T> {
    fn owned(self) -> T;
    fn reference(&self) -> &T;
}

impl RefOrOwned<BlockState> for Arc<BlockState> {
    fn owned(self) -> BlockState {
        BlockState::clone(self.as_ref())
    }
    
    fn reference(&self) -> &BlockState {
        self.as_ref()
    }
}

impl RefOrOwned<BlockState> for &BlockState {
    fn owned(self) -> BlockState {
        BlockState::clone(self)
    }

    fn reference(&self) -> &BlockState {
        *self
    }
}

impl RefOrOwned<BlockState> for BlockState {
    fn owned(self) -> BlockState {
        self
    }

    fn reference(&self) -> &BlockState {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;
    #[test]
    fn registry_test() -> Result<()> {
        let reg = BlockRegistry::new();
        struct DirtBlock;
        struct GrassBlock;
        impl BlockBehavior for DirtBlock {
            fn name(&self) -> &str {
                "dirt"
            }
        }
        impl BlockBehavior for GrassBlock {
            fn name(&self) -> &str {
                "grass"
            }
        }
        struct DebugBlock(&'static str, Rc<RefCell<bool>>);
        impl BlockBehavior for DebugBlock {
            fn name(&self) -> &str {
                self.0
            }
            fn on_register(&self, _registry: &BlockRegistry) {
                self.1.replace(true);
            }
        }
        let _dirt = reg.register_block(DirtBlock)?;
        let _grass = reg.register_block(GrassBlock)?;
        // for on_register call which will replace value with true.
        let hello_cell = Rc::new(RefCell::new(false));
        let _hello = reg.register_block(DebugBlock("hello", hello_cell.clone()))?;
        debug_assert!(hello_cell.take());
        let world_cell = Rc::new(RefCell::new(false));
        let world = reg.register_block(DebugBlock("world", world_cell.clone()))?;
        debug_assert!(world_cell.take());
        let hello1 = reg.register_state(BlockState::new("hello", []))?;
        reg.access_state(StateId::AIR, |state| {
            debug_assert_eq!(state.name(), "air");
        })?;
        reg.access_block(StateId::AIR, |block| {
            debug_assert_eq!(*block.description().as_ref().unwrap(), "A block of air.");
        })?;
        reg.access_block(hello1, |block| {
            debug_assert_eq!(block.name(), "hello");
        })?;
        reg.access_block(world, |block| {
            debug_assert_eq!(block.name(), "world");
        })?;
        reg.access_state(hello1, |state| {
            debug_assert_eq!(state.name(), "hello");
        })?;
        Ok(())
    }
}