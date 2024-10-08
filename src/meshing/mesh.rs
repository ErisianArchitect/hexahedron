use glam::{
    Vec2,
    Vec3,
};

#[derive(Debug, Default, Clone)]
pub struct VoxelMesh {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub texindices: Vec<u32>,
    pub indices: Vec<u32>,
}