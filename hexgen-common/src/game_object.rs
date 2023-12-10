use crate::matrix::Matrix;
use crate::model::Model;
use crate::transform::{Rotation, Scale, Translation};
use crate::vector3::Vector3;

pub struct GameObject {
    model: Model,
    position: Vector3,
    rotation: Vector3,
    scale: Vector3,
    model_matrix: Matrix,
}

impl GameObject {
    pub fn new(model: Model) -> GameObject {
        GameObject {
            model,
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one(),
            model_matrix: GameObject::calculate_model_matrix(&Vector3::zero(), &Vector3::zero(), &Vector3::one()),
        }
    }

    fn calculate_model_matrix(position: &Vector3, rotation: &Vector3, scale: &Vector3) -> Matrix {
        todo!()
    }
}

impl Translation for GameObject {
    fn translate(&mut self, translation: Vector3) {
        self.position += translation;
    }
}

impl Rotation for GameObject {
    fn rotate(&mut self, rotation: Vector3) {
        self.rotation += rotation;
    }
}

impl Scale for GameObject {
    fn scale(&mut self, scale: Vector3) {
        self.scale += scale;
    }
}