use crate::robot::Robot;
#[derive(Default)]
pub struct Matrix {
    pub matrix: Vec::<Vec<f32>>,
}
impl Matrix {
    pub fn new(a:f32,k:f32,n:f32,l:f32) -> Self {
        let mut matrix:Matrix = Matrix{matrix:vec![]};
        matrix.matrix.push(vec![a.cos(),a.sin(),0.0]);
        matrix.matrix.push(vec![-a.sin(),a.cos(),0.0]);
        matrix.matrix.push(vec![l*a.cos()+k,l*a.sin()+n,1.0]);
        matrix
    }
    pub fn moment_position(&self, robot: &mut Robot){
        let matrix = &self.matrix;
        robot.position.x = matrix[0][0] * robot.position.x + matrix[0][1] * robot.position.y + matrix[0][2];
        robot.position.y = matrix[1][0] * robot.position.x + matrix[1][1] * robot.position.y + matrix[1][2];
        }
    }