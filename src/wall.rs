use crate::positionning::{Pos, RotateAngle};


// Enum variants on type aliases are still experimental.
// As soon as it isn't, you can alias `Side` by `Way`;
impl_way!(Side);


#[derive(Debug, Clone)]
pub struct Wall {
    pub pos: Pos,
    pub side: Side
}
