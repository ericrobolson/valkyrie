#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vec3;

impl Vec3 {
    pub fn unit_x() -> Self {
        Self {}
    }
    pub fn unit_z() -> Self {
        Self {}
    }
    pub fn unit_y() -> Self {
        Self {}
    }
}

impl Into<(f32, f32, f32)> for Vec3 {
    fn into(self) -> (f32, f32, f32) {
        println!("TODO: vec3 into");
        (0., 0., 0.)
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Quaterion;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Mat4;

impl Mat4 {
    pub fn view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Self {}
    }

    pub fn as_slice(&self) -> &[f32] {
        &[]
    }
}
