use std::f32::consts::TAU;
use eframe::egui::{remap, Color32, Response, Ui, Vec2};
use egui_plot::{Legend, Line, Plot, PlotPoints};
mod rectangle;
use rectangle::Rectangle;
#[derive(Default, Clone, PartialEq)]
pub struct Robot {
    pub show_grid: bool,
    pub position : Vec2,
    pub base: Rectangle,
    pub a:f32,
    pub k:f32,
    pub n:f32,
    pub l:f32,
    pub prev_position : Vec2,
}

impl Robot {
    fn circle(&self) -> Line {
        let circle_center = self.position;
        let circle_radius = 0.5;
        let n = 512;
        let circle_points: PlotPoints = (0..n)
            .map(|i| {
                let t = remap(i as f64, 0.0..=(n as f64), 0.0..=(TAU as f64));
                [
                circle_center.x as f64 + circle_radius * t.cos(),
                circle_center.y as f64 + circle_radius * t.sin()
                ]
            }
            ).collect();
        Line::new(circle_points)
            .color(Color32::from_rgb(100, 200, 100))
            .name("circle")
            .fill(circle_center.y - circle_radius as f32)
    }
    pub const fn new() -> Self {
        Robot {
            show_grid: true,
            position: Vec2{x: 0.0, y: 0.0},
            a: 0.0,
            k: 0.0,
            n: 0.0,
            l: 0.0,
            base: Rectangle::new(),
            prev_position: Vec2{x: 0.0, y: 0.0},
        }
    }

    pub fn draw(&self, ui: &mut Ui) -> Response {
        let plot = Plot::new("robot")
            .show_grid(self.show_grid)
            .show_axes(true)
            .data_aspect(1.0)
            .legend(Legend::default());
        plot.show(ui, |plot_ui| {
            plot_ui.line(self.circle());
            plot_ui.line(self.base.make_line());
            plot_ui.line(Line::new(PlotPoints::from(vec![(self.base.center.x as f64, self.base.center.y as f64).into(),(self.prev_position.x as f64,self.prev_position.y as f64,).into()])));
            plot_ui.line(Line::new(PlotPoints::new(vec![(self.prev_position.x as f64, self.prev_position.y as f64).into(),(self.position.x as f64,self.position.y as f64).into()])));
        }).response
    }
    pub fn moment_position(&mut self){
        self.position.x = self.l*self.a.to_radians().cos() + self.k;
        self.position.y = self.l*self.a.to_radians().sin() + self.n;
        }
}
