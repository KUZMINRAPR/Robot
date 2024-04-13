use eframe::egui::Pos2;
use egui_plot::{Line, PlotPoints};
#[derive(Default,Clone)]
pub struct Rectangle {
    pub lenght: f64,
    pub width: f64,
    pub start_pos: Pos2,
    pub end_pos: Pos2
}
impl Rectangle {
    pub const fn new() -> Rectangle {
        Rectangle{
            lenght: 2.0,
            width: 0.5,
            start_pos: Pos2{x: -1.0, y: -0.5},
            end_pos: Pos2{x: 1.0, y: -0.5}
        }
    }
    pub fn make_line(&self) -> Line {
        Line::new(self.make_points())
        .fill((self.start_pos.y - self.start_pos.y - self.width as f32) as f32)
        .name("base")
    }
    fn make_points(&self) -> PlotPoints {
        vec![(self.start_pos.x as f64, self.start_pos.y as f64).into(),
        (self.start_pos.x as f64 + self.lenght as f64,self.start_pos.y as f64).into(),
        (self.start_pos.x as f64 + self.lenght as f64,self.start_pos.y as f64 - self.width as f64).into(),
        (self.start_pos.x as f64,self.start_pos.y as f64 - self.width as f64).into(),
        (self.start_pos.x as f64, self.start_pos.y as f64).into(),].into()
    }
}