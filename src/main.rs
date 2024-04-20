use eframe::egui;
use eframe::egui::{Slider, Vec2, Window};
use eframe::run_native;

mod robot;
use eframe::App;
use robot::Robot;
static mut ROBOT: Robot = Robot::new();
static CENTER: Vec2 = Vec2{x: 0.0, y: 0.0};
impl App for Robot {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Window::new("Robot").show(ctx, |ui| {
            unsafe{
            let tmp = ROBOT.clone();
            ui.add(Slider::new(&mut ROBOT.a,0.0..=180.0).text("a"));
            ui.add(Slider::new(&mut ROBOT.k,-10.0..=10.0).text("k"));
            ROBOT.base.center.x = ROBOT.k;
            ui.add(Slider::new(&mut ROBOT.n,0.0..=10.0).text("n"));
            ui.add(Slider::new(&mut ROBOT.l,0.0..=10.0).text("l"));
            if tmp != ROBOT {
                ROBOT.position = CENTER;
                ROBOT.moment_position();
                if tmp.l == ROBOT.l && ROBOT.position.x == ROBOT.base.center.x {
                    ROBOT.prev_position = ROBOT.position;
                }
            }
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