use bytemuck::NoUninit;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, NoUninit)]
#[repr(u8)]
pub enum Cardinal {
    /// -X
    West  = 0,
    /// -Z
    North = 1,
    /// +X
    East  = 2,
    /// +Z
    South = 3,
}

impl Cardinal {
    pub const FORWARD: Cardinal = Cardinal::North;
    pub const BACKWARD: Cardinal = Cardinal::South;
    pub const RIGHT: Cardinal = Cardinal::East;
    pub const LEFT: Cardinal = Cardinal::West;
    /// Ordered: West, East, North, South
    /// West and East, North and South are grouped together for certain desirable effects.
    pub const ALL: [Cardinal; 4] = [
        Cardinal::West,
        Cardinal::East,
        Cardinal::North,
        Cardinal::South,
    ];

    /// Rotates the [Cardinal] direction clockwise by `rotation`.
    pub const fn rotate(self, rotation: i32) -> Self {
        const CARDS: [Cardinal; 4] = [
            Cardinal::West,
            Cardinal::North,
            Cardinal::East,
            Cardinal::South
        ];
        let index = self as i32;
        let rot_index = (index + rotation).rem_euclid(4);
        CARDS[rot_index as usize]
    }

    /// Inverts the [Cardinal] to the opposite direction.
    pub const fn invert(self) -> Self {
        match self {
            Cardinal::West => Cardinal::East,
            Cardinal::East => Cardinal::West,
            Cardinal::North => Cardinal::South,
            Cardinal::South => Cardinal::North,
        }
    }

    /// Gets the [Cardinal] as a single bit based on discriminant.
    pub const fn bit(self) -> u8 {
        1 << self as u8
    }

    pub const fn discriminant(self) -> u8 {
        self as u8
    }

    pub fn iter() -> impl Iterator<Item = Cardinal> {
        Self::ALL.into_iter()
    }
}