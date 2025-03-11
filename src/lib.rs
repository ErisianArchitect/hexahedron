//! Hexahedron is a game engine for making blocky voxel based games.
//! It's functionality is inspired by Minecraft, so there are some concepts that are the same.
//! 
//! # Features
//! * Orient a block using rotation and axis inversion.
//! * Attach metadata to blocks.
//! * Global block update queue.
//! * Region format inspired by Minecraft.
//! # Planned Features
//! * ECS (Entity Component System).
//! * Lighting system similar to Minecraft.
//! * Procedural content generation API.
//! * Loading block meshes from `.gltf` files.
//! * Task scheduler.
//! * (Maybe) Multiplayer server architecture.
//! * Scripting API.

// pub mod collections;
pub mod engine;
// pub mod io;
// pub mod math;
// pub mod meshing;
// pub mod rendering;
// pub mod tag;
pub mod util;
// pub mod voxel;
// pub mod error;
// pub mod macros;
// pub mod prelude;
pub(crate) mod private;

// i=int(not())
// def ch(*args):
//    for arg in args:yield from arg
// def o(v):yield v
// def yt(v,t):yield from (v for _ in range(t))
// def yc(c, t):yield from (c() for _ in range(t))
// cb=lambda a,b:not(((a&i)==i)^((b&i)==i))
// s=lambda f,t,c:t if c else f
// vi=(((v&0xf),((v&0xf0)>>0x4)) for v in range(0xff+i))
// ln=lambda:(s(chr(ord(max(str(not())))-ord(min(str(not())))^i),chr(sum(range(ord(min(str(not())))))),cb(*next(vi))) for _ in range(0xf+i))
// print(''.join(ch(*(ch(ln(),o(chr(0xa))) for _ in range(0xf+i)))))

