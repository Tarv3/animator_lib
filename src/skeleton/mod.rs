pub mod bone;

use skeleton::bone::*;
use pose::*;

#[derive(Debug)]
pub struct Skeleton {
    bones: Vec<Bone>,
}

impl Skeleton {
    pub fn bones_ref(&self) -> &[Bone] {
        self.bones.as_slice()
    }

    pub fn bones_mut_ref(&mut self) -> &mut [Bone] {
        &mut self.bones[..]
    }

    pub fn from_bones(bones: Vec<Bone>) -> Skeleton {
        Skeleton {
            bones,
        }
    }

    pub fn bone_count(&self) -> usize {
        self.bones.len()
    }

    pub fn with_capacity(capacity: usize) -> Skeleton {
        Skeleton {
            bones: Vec::with_capacity(capacity),
        }
    }

    pub fn add_bone(&mut self, bone: Bone) {
        self.bones.push(bone);
    }

    pub fn write_matrices_to_buffer(&self, buffer: &mut [[[f32; 4]; 4]]) -> Result<(), MissingFinalPose> {
        let len = self.bones.len();
        for i in 0..len {
            if i >= buffer.len() {
                break;
            }
            buffer[i] = self.bones[i].get_relative_pose().ok_or(MissingFinalPose)?.matrix().into();
        }
        Ok(())
    }

    pub fn write_poses_to_buffer(&self, buffer: &mut [Pose]) -> Result<(), MissingFinalPose> {
        let len = self.bones.len();
        for i in 0..len {
            if i >= len {
                break;
            }
            buffer[i] = self.bones[i].get_relative_pose().ok_or(MissingFinalPose)?;
        }

        Ok(())
    }

    pub fn reset_inv_bindposes(&mut self) {
        for bone in self.bones.iter_mut() {
            bone.inv_pose = None;
        }
    }

    // Will set the bindposes of all of the bones to their current transformations
    pub fn build_inv_bindposes(&mut self) {
        self.reset_inv_bindposes();
        
        for i in 0..self.bones.len() {
            if self.bones[i].inv_pose.is_none() {
                let mut bone = self.bones[i];
                bone.build_inv_pose(&mut self.bones);
                self.bones[i] = bone;
            }
        }
    }

    // Will set the bindposes of all of the bones to their current transformations
    pub fn build_inv_bindposes_from_final(&mut self) {
        for bone in self.bones.iter_mut() {
            let inv = bone.final_pose.unwrap().inverse();
            bone.inv_pose = Some(inv);
        }
    }

    pub fn reset_bones(&mut self) {
        for bone in &mut self.bones {
            bone.reset();
        }
    }

    // Must be called after updating bone transformations
    pub fn rebuild_poses(&mut self) -> Result<(), MissingInvBindpose> {
        self.reset_bones();
        for i in 0..self.bones.len() {
            if self.bones[i].final_pose.is_some() {
                continue;
            }
            let mut bone = self.bones[i];
            bone.build_pose(&mut self.bones);
            self.bones[i] = bone;
        }

        Ok(())
    }

    // Length of a1 and a2 must be equal to the length of bones
    pub fn interp_animations(&mut self, a1: &[Pose], a2: &[Pose], t: f32) {
        assert!(a1.len() == a2.len() && a1.len() == self.bones.len());

        for (i, bone) in self.bones.iter_mut().enumerate() {
            let pose1 = a1[i];
            let pose2 = a2[i];
            let new_pose = pose_interp(&pose1, &pose2, t);
            bone.pose = new_pose;
        }
    }

    pub fn set_poses(&mut self, poses: &[Pose]) {
        assert!(poses.len() == self.bones.len());

        for i in 0..poses.len() {
            self.bones[i].pose = poses[i];
        }
    }

    pub fn set_base_pose(&mut self, base: Pose) -> Result<(), MissingFinalPose> {
        for bone in self.bones.iter_mut() {
            bone.set_base_pose(base)?;
        }

        Ok(())
    }
    
    #[cfg(debug)]
    pub fn pretty_print(&self) {
        for (i, bone) in self.bones.iter().enumerate() {
            println!("Bone {} = {:?}\n", i, bone);
        }
    }
}
