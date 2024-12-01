use hexahedron::prelude::*;

pub enum Message {
    Interact,
    Move,
    Face(Cardinal),
}