mod perspective;

use hexgen_common::matrix::Matrix;
use hexgen_common::transform::Translation;
use hexgen_common::vector3::Vector3;
use crate::camera::perspective::Perspective;

pub struct Camera {
    position: Vector3,
    direction: Vector3,
    up: Vector3,
    view_matrix: Matrix,
    perspective: Perspective,
}

impl Camera {
    pub fn new(position: Vector3, direction: Vector3, up: Vector3, perspective: Perspective) -> Camera {
        Camera {
            view_matrix: Camera::calculate_view_matrix(&position, &direction, &up),
            position,
            direction,
            up,
            perspective,
        }
    }

    fn calculate_view_matrix(position: &Vector3, direction: &Vector3, up: &Vector3) -> Matrix {
        let f = {
            let f = direction;
            let len = f.x * f.x + f.y * f.y + f.z * f.z;
            let len = len.sqrt();
            Vector3::new(f.x / len, f.y / len, f.z / len)
        };

        let s = Vector3::new(up.y * f.z - up.z * f.y,
                             up.z * f.x - up.x * f.z,
                             up.x * f.y - up.y * f.x);

        let s_norm = {
            let len = s.x * s.x + s.y * s.y + s.z * s.z;
            let len = len.sqrt();
            Vector3::new(s.x / len, s.y / len, s.z / len)
        };

        let u = Vector3::new(f.y * s_norm.z - f.z * s_norm.y,
                             f.z * s_norm.x - f.x * s_norm.z,
                             f.x * s_norm.y - f.y * s_norm.x);

        let p = Vector3::new(-position.x * s_norm.x - position.y * s_norm.y - position.z * s_norm.z,
                             -position.x * u.x - position.y * u.y - position.z * u.z,
                             -position.x * f.x - position.y * f.y - position.z * f.z);

        Matrix([
            [s_norm.x, u.x, f.x, 0.0],
            [s_norm.y, u.y, f.y, 0.0],
            [s_norm.z, u.z, f.z, 0.0],
            [p.x, p.y, p.z, 1.0],
        ])
    }
}

impl Translation for Camera {
    fn translate(&mut self, translation: Vector3) {
        self.position += translation;
        self.view_matrix = Camera::calculate_view_matrix(&self.position, &self.direction, &self.up);
    }
}