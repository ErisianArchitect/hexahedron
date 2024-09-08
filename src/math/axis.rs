use bytemuck::NoUninit;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, NoUninit)]
#[repr(u8)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2
}