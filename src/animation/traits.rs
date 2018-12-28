use pose::*;
use glm::{Vec3, Quat};

pub enum Targets<'a> {
    // The index of each pose corresponds to the index of the bone it targets
    InOrder,

    // Relates the index of the pose to the index of the bone it targets
    Specified(&'a [usize]),
}

pub trait Animation {

    fn sample_times(&self) -> &[f32];

    fn get_frame(&self, frame: usize) -> Option<(&[Pose], f32)>;

    fn get_targets<'a>(&'a self) -> Targets<'a>;

    fn frames(&self) -> usize {
        self.sample_times().len()
    }

    fn duration(&self) -> Option<f32> {
        self.sample_times().last().map(|x| *x)
    }

    fn first_frame(&self) -> Option<(&[Pose], f32)> {
        self.get_frame(0)
    }

    fn last_frame(&self) -> Option<(&[Pose], f32)> {
        let frames = self.frames();
        if frames > 0 {
            self.get_frame(frames - 1)
        }
        else {
            None
        }
    }
} 

pub trait AnimationTarget {
    fn set_pose(&mut self, pose: Pose);

    fn get_pose_mut(&mut self) -> &mut Pose;

    fn get_pose(&self) -> Pose;

    fn add_pose(&mut self, pose: Pose) {
        let pose = pose * self.get_pose();
        self.set_pose(pose)
    }

    fn add_translation(&mut self, translation: Vec3) {
        self.get_pose_mut().translation += translation;
    }

    fn add_rotation(&mut self, rotation: Quat) {
        self.get_pose_mut().add_rotation(rotation);
    }
}

impl AnimationTarget for Pose {
    fn set_pose(&mut self, pose: Pose) {
        *self = pose;
    }

    fn get_pose_mut(&mut self) -> &mut Pose {
        self
    }

    fn get_pose(&self) -> Pose {
        *self
    }

    fn add_pose(&mut self, pose: Pose) {
        *self = pose * *self;
    }

    
}