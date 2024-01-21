use crate::matrix::Matrix;
use crate::model::Model;
use crate::transform::{Rotation, Scale, Translation};
use crate::vector3::Vector3;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GameObject {
    name: String,
    pub model: Rc<RefCell<Model>>,
    position: Vector3,
    rotation: Vector3,
    scale: Vector3,
    pub model_matrix: Matrix,
}

impl GameObject {
    pub fn new(name: String, model: Rc<RefCell<Model>>) -> GameObject {
        // info!("Created Game Object '{}'", name);
        GameObject {
            name,
            model,
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one(),
            model_matrix: GameObject::calculate_model_matrix(
                &Vector3::zero(),
                &Vector3::zero(),
                &Vector3::one(),
            ),
        }
    }

    fn calculate_model_matrix(position: &Vector3, rotation: &Vector3, scale: &Vector3) -> Matrix {
        let scale_matrix = Matrix([
            [scale.x, 0.0, 0.0, 0.0],
            [0.0, scale.y, 0.0, 0.0],
            [0.0, 0.0, scale.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let yaw_matrix = Matrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, rotation.x.cos(), rotation.x.sin(), 0.0],
            [0.0, -rotation.x.sin(), rotation.x.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let pitch_matrix = Matrix([
            [rotation.y.cos(), 0.0, -rotation.y.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [rotation.y.sin(), 0.0, rotation.y.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let roll_matrix = Matrix([
            [rotation.z.cos(), rotation.z.sin(), 0.0, 0.0],
            [-rotation.z.sin(), rotation.z.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let translate_matrix = Matrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [position.x, position.y, position.z, 1.0],
        ]);

        let matrix = translate_matrix * yaw_matrix * pitch_matrix * roll_matrix * scale_matrix;
        matrix
    }

    pub fn update_matrix(&mut self) {
        self.model_matrix =
            GameObject::calculate_model_matrix(&self.position, &self.rotation, &self.scale);
    }
}

impl<'a> Translation for GameObject {
    fn translate(&mut self, translation: Vector3) {
        self.position = Vector3::new(
            translation.x / self.scale.x,
            translation.y / self.scale.y,
            translation.z / self.scale.z,
        );
        self.update_matrix();
    }
}

impl<'a> Rotation for GameObject {
    fn rotate(&mut self, rotation: Vector3) {
        self.rotation = rotation;
        self.update_matrix();
    }
}

impl<'a> Scale for GameObject {
    fn scale(&mut self, scale: Vector3) {
        self.scale = scale;
        self.position = Vector3::new(
            self.position.x / self.scale.x,
            self.position.y / self.scale.y,
            self.position.z / self.scale.z,
        );
        self.update_matrix();
    }
}

