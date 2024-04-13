use eframe::egui::Pos2;
use egui_plot::{Line, PlotPoints};
#[derive(Default,Clone)]
pub struct Rectangle {
    pub lenght: f64,
    pub width: f64,
    pub center: Pos2
}
impl Rectangle {
    pub const fn new() -> Rectangle {
        Rectangle{
            lenght: 2.0,
            width: 0.5,
            center: Pos2{x: 0.0, y: -0.5}
        }
    }
    pub fn make_line(&self) -> Line {
        Line::new(self.make_points())
        .fill((self.center.y - self.center.y - self.width as f32) as f32)
        .name("base")
    }
    fn make_points(&self) -> PlotPoints {
        vec![(self.center.x as f64 - self.lenght as f64, self.center.y as f64).into(),
        (self.center.x as f64 + self.lenght as f64,self.center.y as f64).into(),
        (self.center.x as f64 + self.lenght as f64,self.center.y as f64 - self.width as f64).into(),
        (self.center.x as f64 - self.lenght as f64,self.center.y as f64 - self.width as f64).into(),
        (self.center.x as f64 - self.lenght as f64, self.center.y as f64).into(),].into()
    }
}