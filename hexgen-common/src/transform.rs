use crate::vector3::Vector3;

pub trait Translation {
    fn translate(&mut self, translation: Vector3);
}

pub trait Rotation {
    fn rotate(&mut self, rotation: Vector3);
}

pub trait Scale {
    fn scale(&mut self, scale: Vector3);
}
