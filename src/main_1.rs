use eframe::egui::{self, Ui};
use eframe::egui::{Slider, Vec2, Window};
use eframe::run_native;

mod robot;
mod matrix;
use eframe::App;
use egui_plot::{Line, PlotPoints,Plot};
use robot::{Robot,Message};
static mut ROBOT: Robot = Robot::new();
static CENTER: Vec2 = Vec2{x: 0.0, y: 0.0};
impl App for Robot {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        fn robot_position(robot: &mut Robot) {
            match robot.message {
                Message::CornerChanged => {
                    let radius = (robot.position.x.powi(2) + robot.position.y.powi(2)).sqrt();
                    robot.position.x = robot.a.to_radians().cos() * radius + CENTER.x;
                    robot.position.y = robot.a.to_radians().sin() * radius + CENTER.y;
                    robot.message = Message::None;
                    robot.prev_position = robot.position;
                }
                Message::LenghtChanged => {
                    robot.position.x = (robot.l + robot.position.x) * robot.a.to_radians().cos();
                    robot.position.y = (robot.l + robot.position.x) * robot.a.to_radians().sin();
                    robot.message = Message::None;
                }
                Message::BasePositionChanged => {
                    robot.position.x = robot.base.center.x;
                    robot.message = Message::None;
                    robot.prev_position = robot.position;
                }
                Message::YPositionChanged => {
                    robot.prev_position = robot.position;
                }
                _ => {}
            }
        }
        Window::new("Robot").show(ctx, |ui| {
            unsafe{
            let tmp = ROBOT.clone();
            ui.add(Slider::new(&mut ROBOT.a,0.0..=180.0).text("a"));
            if tmp.a != ROBOT.a {ROBOT.message = Message::CornerChanged;}
            ui.add(Slider::new(&mut ROBOT.base.center.x,-10.0..=10.0).text("k"));
            if tmp.base.center.x != ROBOT.base.center.x {ROBOT.message = Message::BasePositionChanged;}
            ui.add(Slider::new(&mut ROBOT.position.y,0.0..=10.0).text("n"));
            if tmp.position.y != ROBOT.position.y {ROBOT.message = Message::YPositionChanged;}
            ui.add(Slider::new(&mut ROBOT.l,0.0..=10.0).text("l"));
            if tmp.l != ROBOT.l {ROBOT.message = Message::LenghtChanged;}
            robot_position(&mut ROBOT);
            ROBOT.draw(ui);
        }
    });
    }
}
fn main() -> Result<(), eframe::Error> {
    
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    run_native("My Robot", options, Box::new(|_cc| Box::new(Robot::new())))
}