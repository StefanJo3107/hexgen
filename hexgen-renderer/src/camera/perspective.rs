use tracing::info;
use hexgen_common::matrix::Matrix;

pub struct Perspective {
    fov: f32,
    z_far: f32,
    z_near: f32,
    pub perspective_matrix: Matrix,
    width: f32,
    height: f32,
}

impl Perspective {
    pub fn new(fov: f32, z_far: f32, z_near: f32, width: f32, height: f32) -> Perspective {
        Perspective {
            fov,
            z_far,
            z_near,
            perspective_matrix: Perspective::calculate_matrix(fov, z_far, z_near, width, height),
            width,
            height,
        }
    }

    pub fn update(&mut self, width: f32, height: f32) {
        self.perspective_matrix = Perspective::calculate_matrix(self.fov, self.z_near, self.z_far, width, height);
        self.width = width;
        self.height = height;
        info!("Updated perspective matrix");
    }

    fn calculate_matrix(fov: f32, z_near: f32, z_far: f32, width: f32, height: f32) -> Matrix {
        let aspect_ratio = height / width;
        let f = 1.0 / (fov / 2.0).tan();

        Matrix([
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (z_far + z_near) / (z_far - z_near), 1.0],
            [0.0, 0.0, -(2.0 * z_far * z_near) / (z_far - z_near), 0.0],
        ])
    }
}