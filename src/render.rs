#![allow(unused)]

pub trait Render {
    fn render(&self);
    fn render_box(&self) -> RenderBox;
}

impl RenderBox {
    pub fn op_x(&self) -> f32 {
        self.x + self.width
    }
    pub fn op_y(&self) -> f32 {
        self.y + self.height
    }
}

pub struct RenderBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
