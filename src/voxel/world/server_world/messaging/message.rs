use serde::{Deserialize, Serialize};

use crate::voxel::block::block_state::BlockState;


/// Network Message.  
/// Messaging protocol for server/client relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetMsg {
    SetBlock(Box<((i32, i32, i32), BlockState)>),
    
}

/// Network Response.
pub enum NetResp {

}

impl NetMsg {
    pub fn set_block(coord: (i32, i32, i32), state: BlockState) -> Self {
        Self::SetBlock(Box::new((coord, state)))
    }
}