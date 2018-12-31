pub mod error;

use self::error::*;
use pose::*;
use std::error::Error;

pub type SkeletalPose = Vec<Pose>;

pub struct Skeleton {
    tree: Vec<Option<usize>>,
    pose: SkeletalPose,
    world_pose: Vec<Option<Pose>>,
    inv_bind_pose: Vec<Option<Pose>>,
}

impl Skeleton {
    pub fn from_tree_pose(tree: Vec<Option<usize>>, pose: SkeletalPose) -> Skeleton {
        assert!(tree.len() == pose.len());

        let world_pose = vec![None; tree.len()];
        let inv_bind_pose = vec![None; tree.len()];

        Skeleton {
            tree,
            pose,
            world_pose,
            inv_bind_pose
        }
    }

    pub fn bone_count(&self) -> usize {
        self.tree.len()
    }

    pub fn tree_ref(&self) -> &[Option<usize>] {
        self.tree.as_slice()
    }

    pub fn pose_ref(&self) -> &[Pose] {
        self.pose.as_slice()
    }

    pub fn pose_ref_mut(&mut self) -> &mut [Pose] {
        &mut self.pose[..]
    }

    pub fn world_pose_ref(&self) -> &[Option<Pose>] {
        self.world_pose.as_slice()
    }

    pub fn map_world_poses<'a, T, M: 'a + FnMut(Option<Pose>) -> T>(&'a self, mut map: M) -> impl Iterator<Item = T> + 'a {
        self.world_pose.iter().cloned().map(move |x| map(x))
    }

    pub fn world_poses_to_matrices<'a>(&'a self) -> impl Iterator<Item = Option<[[f32; 4]; 4]>> + 'a {
        self.world_pose.iter().map(|x| x.map(|x| x.matrix().into()))
    }

    pub fn output_poses<'a>(&'a self) -> impl Iterator<Item = Result<Pose, Box<Error>>> + 'a {
        (0..self.tree.len()).map(move |i| {
            let world = self.world_pose[i].ok_or(MissingFinalPose)?;
            let inv = self.inv_bind_pose[i].ok_or(MissingInvBindpose)?;

            Ok(world * inv)
        })
    }

    pub fn output_matrices<'a>(&'a self) -> impl Iterator<Item = Result<[[f32; 4]; 4], Box<Error>>> + 'a {
        self.output_poses().map(|x| x.map(|x| x.matrix().into()))
    }

    // Only checks that there is enough poses to fill the inv bind poses
    pub fn set_inv_bind_pose_iter(&mut self, mut poses: impl Iterator<Item = Pose>) -> Result<(), MissingInvBindpose> {
        for bp in self.inv_bind_pose.iter_mut() {
            let pose = poses.next().ok_or(MissingInvBindpose)?;
            *bp = Some(pose);
        }

        Ok(())
    } 

    pub fn joint_world_pose(&mut self, joint_id: usize) -> Option<Pose> {
        if joint_id >= self.tree.len() {
            return None;
        }

        if let Some(pose) = self.world_pose[joint_id] {
            return Some(pose);
        }

        let mut pose = self.pose[joint_id];

        match self.tree[joint_id] {
            Some(parent) => {
                let parent_pose = self.joint_world_pose(parent)?;
                pose = parent_pose * pose;
            }
            None => {}
        }

        self.world_pose[joint_id] = Some(pose);
        Some(pose)
    }

    pub fn build_world_poses(&mut self) {
        self.reset_world_poses();

        for i in 0..self.tree.len() {
            self.joint_world_pose(i);
        }
    }

    pub fn reset_world_poses(&mut self) {
        for pose in self.world_pose.iter_mut() {
            *pose = None;
        }
    }

    pub fn joint_inv_bind_pose(&mut self, joint_id: usize) -> Option<Pose> {
        if joint_id >= self.tree.len() {
            return None;
        }

        if let Some(pose) = self.inv_bind_pose[joint_id] {
            return Some(pose);
        }

        let mut pose = self.pose[joint_id].inverse();

        match self.tree[joint_id] {
            Some(parent) => {
                let parent_pose = self.joint_inv_bind_pose(parent)?;
                pose = pose * parent_pose;
            }
            None => {}
        }

        self.inv_bind_pose[joint_id] = Some(pose);
        Some(pose)
    }

    pub fn reset_inv_bind_poses(&mut self) {
        for pose in self.inv_bind_pose.iter_mut() {
            *pose = None;
        }
    }

    pub fn build_inv_bind_poses(&mut self) {
        self.reset_inv_bind_poses();

        for i in 0..self.tree.len() {
            self.joint_inv_bind_pose(i);
        }
    }
}
