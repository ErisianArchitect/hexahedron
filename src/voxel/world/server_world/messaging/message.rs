use glam::*;

use crate::{io::{Readable, Writeable}, tag::Tag, voxel::block::block_state::BlockState};

/* 
The clients will sent out NetMsg to the server for actions taken by the client.
The server will receive those message, and may respond with NetResp.

*/

#[derive(Debug)]
pub struct Positioned<T: Readable + Writeable> {
    position: IVec3,
    value: T,
}

impl<T: Readable + Writeable> Positioned<T> {
    pub const fn new(position: (i32, i32, i32), value: T) -> Self {
        let (x, y, z) = position;
        let position = ivec3(x, y, z);
        Self {
            position,
            value,
        }
    }

    pub const fn position(&self) -> IVec3 {
        self.position
    }

    pub const fn value(&self) -> &T {
        &self.value
    }
}

/// Network Message.  
/// Messaging protocol for server/client relationship.
#[derive(Debug, Clone)]
pub enum NetMsg {
    SetBlock(Box<Positioned<BlockState>>),
    SetTag(Box<Positioned<Tag>>),
    SetEnabled(Box<Positioned<bool>>),
    MovePlayer(Box<Vec3>),
    PlayerFace(Box<Vec3>),
}

/// Network Response.
pub enum NetResp {

}

impl NetMsg {
    pub fn set_block(coord: IVec3, state: BlockState) -> Self {
        Self::SetBlock(Box::new((coord, state)))
    }

    pub fn set_tag(coord: IVec3, tag: Tag) -> Self {
        Self::SetTag(Box::new((coord, tag)))
    }

    pub fn move_player(position: Vec3) -> Self {
        Self::MovePlayer(Box::new(position))
    }

    pub fn player_face(direction: Vec3) -> Self {
        Self::PlayerFace(Box::new(direction))
    }
}