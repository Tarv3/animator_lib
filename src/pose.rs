use glm;
use glm::*;
use math::*;
use std::ops::{Mul, MulAssign};

// Mixes in the form "a * (1 - f) + b * f"
pub fn pose_interp_fast(a: &Pose, b: &Pose, f: f32) -> Pose {
    let f = maxf32(minf32(f, 1.0), 0.0);
    let omf = 1.0 - f;
    let translation = a.translation * omf + b.translation * f;
    let rotation;
    if dot(&a.rotation.coords, &b.rotation.coords) < 0.0 {
        rotation = quat_fast_mix(&-a.rotation, &b.rotation, f);
    } else {
        rotation = quat_fast_mix(&a.rotation, &b.rotation, f);
    }
    let scale = a.scale * omf + b.scale * f;

    Pose {
        translation,
        rotation,
        scale,
    }
}

// Mixes in the form "a * (1 - f) + b * f"
pub fn pose_interp(a: &Pose, b: &Pose, f: f32) -> Pose {
    let f = maxf32(minf32(f, 1.0), 0.0);
    let omf = 1.0 - f;
    let translation = a.translation * omf + b.translation * f;
    let rotation;
    if dot(&a.rotation.coords, &b.rotation.coords) < 0.0 {
        rotation = quat_slerp(&-a.rotation, &b.rotation, f);
    } else {
        rotation = quat_slerp(&a.rotation, &b.rotation, f);
    }
    let scale = a.scale * omf + b.scale * f;

    Pose {
        translation,
        rotation,
        scale,
    }
}

// Interpolates in the form a * (1 - val) + b * val and writes the poses into to
// all array slices must be the same length
pub fn interpolate_poses(a: &[Pose], b: &[Pose], to: &mut [Pose], val: f32) {
    assert!(a.len() == b.len() && a.len() == to.len());

    for i in 0..a.len() {
        to[i] = pose_interp(&a[i], &b[i], val);
    }
}

// Interpolates between a and b where each pose has its own weight where the final poses
// are to[i] = a[i] * (1 - interp_factors[i]) + b[i] * interp_factors[i]
pub fn weighted_pose_interp(a: &[Pose], b: &[Pose], to: &mut [Pose], interp_factors: &[f32]) {
    assert!(a.len() == b.len() && a.len() == to.len() && a.len() == interp_factors.len());

    for i in 0..a.len() {
        to[i] = pose_interp(&a[i], &b[i], interp_factors[i]);
    }
}

// Mixes a collection of poses with weights linearly
// may not work as intended
pub fn n_pose_interp(poses: &[Pose], weights: &mut [f32]) -> Pose {
    assert!(poses.len() == weights.len() && weights.len() > 1);

    let sum: f32 = weights.iter().sum();
    let one_on_sum = 1.0 / sum;
    let mut current_pose = poses[0];
    let mut current_weight = weights[0];

    for (i, weight) in weights.iter().map(|a| *a * one_on_sum).enumerate().skip(1) {
        let i = i + 1;
        let next_pose = poses[i];
        current_weight += weight;
        let interp_factor = weight / current_weight;
        current_pose = pose_interp(&current_pose, &next_pose, interp_factor);
    }

    current_pose
}

pub fn write_poses(a: &[Pose], to: &mut [Pose]) {
    assert!(a.len() == to.len());

    for i in 0..to.len() {
        to[i] = a[i];
    }
}

// Applies pose * base to all poses
pub fn apply_base_pose(base: Pose, poses: &mut [Pose]) {
    for pose in poses.iter_mut() {
        *pose = *pose * base;
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Pose {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Pose {
    pub fn pose_identity() -> Pose {
        let translation = Vec3::new(0.0, 0.0, 0.0);
        let rotation = quat_identity();
        let scale = Vec3::new(1.0, 1.0, 1.0);

        Pose {
            translation,
            rotation,
            scale,
        }
    }

    pub fn only_trans(translation: Vec3) -> Pose {
        Pose {
            translation,
            rotation: quat_identity(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn only_scale(scale: Vec3) -> Pose {
        Pose {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: quat_identity(),
            scale,
        }
    }

    pub fn only_rot(rotation: Quat) -> Pose {
        Pose {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation,
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn without_trans(rotation: Quat, scale: Vec3) -> Pose {
        Pose {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation,
            scale,
        }
    }

    pub fn without_rot(translation: Vec3, scale: Vec3) -> Pose {
        Pose {
            translation,
            rotation: quat_identity(),
            scale,
        }
    }

    pub fn without_scale(translation: Vec3, rotation: Quat) -> Pose {
        Pose {
            translation,
            rotation,
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn rotation_mat(&self) -> Mat4 {
        quat_to_mat4(&self.rotation)
    }

    pub fn translation_mat(&self) -> Mat4 {
        translation(&self.translation)
    }

    pub fn add_translation_to_mat(&self, m: &mut Mat4) {
        m[(0, 3)] = self.translation.x;
        m[(1, 3)] = self.translation.y;
        m[(2, 3)] = self.translation.z;
    }

    // Creates a matrix in the form of translation * rotation * scale
    pub fn matrix(&self) -> Mat4 {
        let mut rotation = self.rotation_mat();
        self.add_translation_to_mat(&mut rotation);
        glm::scale(&rotation, &self.scale)
    }

    // Assumes matrix is an affine transformation with no shearing
    pub fn from_matrix(m: &Mat4) -> Pose {
        let translation = make_vec3(value_ptr(&column(m, 3)));
        let scale_rot = mat4_to_mat3(m);

        let (scale, rot) = separate_rot_scale(&scale_rot);
        let rotation = mat3_to_quat(&rot);
        Pose {
            translation,
            rotation,
            scale,
        }
    }

    pub fn inverse(&self) -> Pose {
        let rotation = quat_inverse(&self.rotation);
        let translation = -quat_rotate_vec3(&rotation, &self.translation);
        let scale = vec3(1.0 / self.scale.x, 1.0 / self.scale.y, 1.0 / self.scale.z);

        Pose {
            translation,
            rotation,
            scale,
        }
    }

    pub fn transform_point(&self, mut point: Vec3) -> Vec3 {
        point = vec3(
            point.x * self.scale.x,
            point.y * self.scale.y,
            point.z * self.scale.z,
        );
        point = quat_rotate_vec3(&self.rotation, &point);
        point + self.translation
    }
}

// The multiplication only scales along the transformed axis as to never shear
// will create a pose in the form of rhs then lhs.
impl Mul<Pose> for Pose {
    type Output = Pose;
    fn mul(self, rhs: Pose) -> Pose {
        let rot_trans = quat_rotate_vec(&self.rotation, &vec3_to_vec4(&rhs.translation));
        let translation = vec4_to_vec3(&rot_trans) + self.translation;
        let rotation = self.rotation * rhs.rotation;
        let scale = Vec3::new(
            self.scale.x * rhs.scale.x,
            self.scale.y * rhs.scale.y,
            self.scale.z * rhs.scale.z,
        );

        Pose {
            translation,
            rotation,
            scale,
        }
    }
}

impl MulAssign<Pose> for Pose {
    fn mul_assign(&mut self, rhs: Pose) {
        *self = &*self * &rhs;
    }
}

impl<'a> Mul<&'a Pose> for Pose {
    type Output = Pose;
    fn mul(self, rhs: &Pose) -> Pose {
        let rot_trans = quat_rotate_vec(&self.rotation, &vec3_to_vec4(&rhs.translation));
        let translation = vec4_to_vec3(&rot_trans) + self.translation;
        let rotation = self.rotation * rhs.rotation;
        let scale = Vec3::new(
            self.scale.x * rhs.scale.x,
            self.scale.y * rhs.scale.y,
            self.scale.z * rhs.scale.z,
        );

        Pose {
            translation,
            rotation,
            scale,
        }
    }
}

impl<'a> MulAssign<&'a Pose> for Pose {
    fn mul_assign(&mut self, rhs: &Pose) {
        *self = &*self * rhs;
    }
}

impl<'a> Mul<Pose> for &'a Pose {
    type Output = Pose;
    fn mul(self, rhs: Pose) -> Pose {
        let rot_trans = quat_rotate_vec(&self.rotation, &vec3_to_vec4(&rhs.translation));
        let translation = vec4_to_vec3(&rot_trans) + self.translation;
        let rotation = self.rotation * rhs.rotation;
        let scale = Vec3::new(
            self.scale.x * rhs.scale.x,
            self.scale.y * rhs.scale.y,
            self.scale.z * rhs.scale.z,
        );

        Pose {
            translation,
            rotation,
            scale,
        }
    }
}

impl<'a, 'b> Mul<&'a Pose> for &'b Pose {
    type Output = Pose;
    fn mul(self, rhs: &'a Pose) -> Pose {
        let rot_trans = quat_rotate_vec(&self.rotation, &vec3_to_vec4(&rhs.translation));
        let translation = vec4_to_vec3(&rot_trans) + self.translation;
        let rotation = self.rotation * rhs.rotation;
        let scale = Vec3::new(
            self.scale.x * rhs.scale.x,
            self.scale.y * rhs.scale.y,
            self.scale.z * rhs.scale.z,
        );

        Pose {
            translation,
            rotation,
            scale,
        }
    }
}

impl From<Mat4> for Pose {
    fn from(mat: Mat4) -> Pose {
        Pose::from_matrix(&mat)
    }
}

impl Into<Mat4> for Pose {
    fn into(self) -> Mat4 {
        self.matrix()
    }
}
