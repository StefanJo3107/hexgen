use hexgen_common::vector3::Vector3;

pub struct DirectionalLight {
    pub direction: Vector3,
}

impl DirectionalLight {
    pub fn new(direction: Vector3) -> DirectionalLight {
        DirectionalLight {
            direction,
        }
    }
}