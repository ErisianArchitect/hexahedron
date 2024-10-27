use hashbrown::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use super::blockstate::BlockState;
use super::{block::BlockBehavior, id::{BlockId, StateId}};
use super::error::{Error, Result};

// main

#[derive(Default, )]
struct InnerBlockRegistry {
    blocks: Vec<Arc<dyn BlockBehavior>>,
    block_lookup: HashMap<String, BlockId>,
    states: Vec<Arc<BlockState>>,
    block_ids: Vec<BlockId>,
    state_lookup: HashMap<Arc<BlockState>, StateId>,
}

#[derive(Clone)]
pub struct BlockRegistry(Arc<RwLock<InnerBlockRegistry>>);

impl BlockRegistry {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(InnerBlockRegistry::default())))
    }

    fn read_lock<'a>(&'a self) -> Result<RwLockReadGuard<'a, InnerBlockRegistry>> {
        self.0.read().map_err(|_| Error::FailedToLock)
    }

    fn write_lock<'a>(&'a self) -> Result<RwLockWriteGuard<'a, InnerBlockRegistry>> {
        self.0.write().map_err(|_| Error::FailedToLock)
    }

    pub fn register_block<B: BlockBehavior + 'static + Sized>(&self, block: B) -> Result<BlockId> {
        let mut reg = self.write_lock()?;
        if reg.block_lookup.contains_key(block.name()) {
            return Err(Error::DuplicateBlockEntry(block.name().to_owned()));
        }
        if reg.blocks.len() > u32::MAX as usize {
            return Err(Error::RegistryOverflow);
        }
        let block_id = BlockId(reg.blocks.len() as u32);
        let block = Arc::new(block);
        reg.blocks.push(block.clone());
        reg.block_lookup.insert(block.name().to_owned(), block_id);
        drop(reg);
        block.on_register();
        Ok(block_id)
    }

    pub fn register_state<S: RefOrOwned<BlockState>>(&self, state: S) -> Result<StateId> {
        let mut reg = self.write_lock()?;
        if let Some(&state_id) = reg.state_lookup.get(state.reference()) {
            Ok(state_id)
        } else {
            let state = state.owned();
            let state_id = StateId(reg.states.len() as u32);
            if let Some(&block_id) = reg.block_lookup.get(state.name()) {
                reg.block_ids.push(block_id);
            } else {
                return Err(Error::BlockNotFound(state.name().to_owned()));
            }
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

pub trait RefOrOwned<T> {
    fn owned(self) -> T;
    fn reference(&self) -> &T;
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
            fn on_register(&self) {
                println!("dirt block registered.");
            }
        }
        impl BlockBehavior for GrassBlock {
            fn name(&self) -> &str {
                "grass"
            }
            fn on_register(&self) {
                println!("grass block registered.");
            }
        }
        struct DebugBlock(&'static str);
        impl BlockBehavior for DebugBlock {
            fn name(&self) -> &str {
                &self.0
            }
            fn on_register(&self) {
                println!("{} block registered.", self.name());
            }
        }
        let _dirt = reg.register_block(DirtBlock)?;
        let _grass = reg.register_block(GrassBlock)?;
        let _hello = reg.register_block(DebugBlock("hello"))?;
        let world = reg.register_block(DebugBlock("world"))?;

        let hello1 = reg.register_state(BlockState::new("hello", []))?;
        reg.access_block(hello1, |block| {
            println!("block: {}", block.name());
        })?;
        reg.access_block(world, |block| {
            println!("block: {}", block.name());
        })?;
        reg.access_state(hello1, |state| {
            println!("state: {}", state.name());
        })?;
        Ok(())
    }
}