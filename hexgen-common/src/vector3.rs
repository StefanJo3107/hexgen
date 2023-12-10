use std::ops;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}


impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            x,
            y,
            z,
        }
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }

    pub fn one() -> Vector3 {
        Vector3 {
            x: 1f32,
            y: 1f32,
            z: 1f32,
        }
    }

    pub fn right() -> Vector3 {
        Vector3 {
            x: 1f32,
            y: 0f32,
            z: 0f32,
        }
    }

    pub fn left() -> Vector3 {
        Vector3 {
            x: -1f32,
            y: 0f32,
            z: 0f32,
        }
    }

    pub fn up() -> Vector3 {
        Vector3 {
            x: 0f32,
            y: 1f32,
            z: 0f32,
        }
    }

    pub fn down() -> Vector3 {
        Vector3 {
            x: 0f32,
            y: -1f32,
            z: 0f32,
        }
    }

    pub fn forward() -> Vector3 {
        Vector3 {
            x: 0f32,
            y: 0f32,
            z: 1f32,
        }
    }

    pub fn back() -> Vector3 {
        Vector3 {
            x: 0f32,
            y: 0f32,
            z: -1f32,
        }
    }
}