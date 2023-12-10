use hexgen_common::vector3::Vector3;

pub struct DirectionalLight {
    direction: Vector3,
    ambient_color: Vector3,
    diffuse_color: Vector3,
    specular_color: Vector3,
}

impl DirectionalLight {
    pub fn new(direction: Vector3, ambient_color: Vector3, diffuse_color: Vector3, specular_color: Vector3) -> DirectionalLight {
        DirectionalLight {
            direction,
            ambient_color,
            diffuse_color,
            specular_color,
        }
    }
}