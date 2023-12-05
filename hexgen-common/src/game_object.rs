use crate::model::Model;
use crate::vector3::Vector3;

pub struct GameObject {
    model: Model,
    position: Vector3,
    rotation: Vector3,
    scale: Vector3,
}

impl GameObject {
    pub fn new(model: Model) -> GameObject {
        GameObject {
            model,
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one(),
        }
    }
}