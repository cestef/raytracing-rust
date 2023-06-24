use super::vec::Color;

#[derive(Debug)]
pub struct Res {
    pub buffers: Vec<Vec<Color>>,
    pub start: i32,
}
