type Num = f32;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vec3 {
    pub x: Num,
    pub y: Num,
    pub z: Num,
}

impl Vec3 {
    pub fn new(x: Num, y: Num, z: Num) -> Self {
        Self { x, y, z }
    }

    pub fn unit_x() -> Self {
        Self {
            x: 1.,
            y: 0.,
            z: 0.,
        }
    }
    pub fn unit_y() -> Self {
        Self {
            x: 0.,
            y: 1.,
            z: 0.,
        }
    }
    pub fn unit_z() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 1.,
        }
    }
}

impl Into<(f32, f32, f32)> for Vec3 {
    fn into(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}

impl Into<cgmath::Point3<f32>> for Vec3 {
    fn into(self) -> cgmath::Point3<f32> {
        cgmath::Point3::new(self.x, self.y, self.z)
    }
}

impl Into<cgmath::Vector3<f32>> for Vec3 {
    fn into(self) -> cgmath::Vector3<f32> {
        cgmath::Vector3::new(self.x, self.y, self.z)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
