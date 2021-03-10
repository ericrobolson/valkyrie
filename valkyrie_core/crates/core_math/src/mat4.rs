use crate::Vec3;
use cgmath::{Vector3, Zero};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4 {
    mat: cgmath::Matrix4<f32>,
}

impl Mat4 {
    pub fn view_matrix(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let eye2: cgmath::Vector3<f32> = eye.into();
        let target: cgmath::Vector3<f32> = target.into();

        let vm = Self {
            mat: cgmath::Matrix4::look_to_rh(eye.into(), target - eye2, up.into()),
        };

        println!("{:?}", vm);
        vm
    }

    pub fn as_slice(&self) -> &[f32] {
        let r: &[f32; 16] = self.mat.as_ref();
        r
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self {
            mat: cgmath::Matrix4::zero(),
        }
    }
}
