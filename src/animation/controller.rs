use super::animator::Animator;
use super::traits::*;
use std::{fmt, error};

pub enum InstanceType {
    AllAdd,
    AllWrite,
    RotationAdd, 
    RotationWrite,
}

pub struct AnimationInstance {
    pub animator: Animator,
    animation_index: usize,
    instance_type: InstanceType,
}

impl AnimationInstance {
    pub fn new(animator: Animator, index: usize, instance_type: InstanceType) -> AnimationInstance {
        AnimationInstance {
            animator,
            animation_index: index,
            instance_type
        }
    }

    pub fn update_frames<A: Animation, L: AnimationLibrary<A>>(&mut self, library: &L) -> Result<(), MissingAnimationError> {
        let animation = library.get_animation(self.animation_index).ok_or(MissingAnimationError { animation: self.animation_index })?;
        let sample_times = animation.sample_times();

        self.animator.update_frames(sample_times);
        Ok(())
    }

    pub fn update_pose<A: Animation, L: AnimationLibrary<A>, T: AnimationTarget>(&self, library: &L, targets: &mut [T]) -> Result<(), Box<error::Error>> {
        let animation = library.get_animation(self.animation_index).ok_or(MissingAnimationError { animation: self.animation_index })?;
        match &self.instance_type {
            InstanceType::RotationWrite => self.animator.write_rotations(animation, targets)?,
            InstanceType::RotationAdd => self.animator.add_rotations(animation, targets)?,
            InstanceType::AllAdd => self.animator.add_pose(animation, targets)?,
            InstanceType::AllWrite => self.animator.write_pose(animation, targets)?,
        }

        Ok(())
    }
}

pub struct Controller {
    animations: Vec<AnimationInstance>, 
}   

impl Controller {
    pub fn new() -> Controller {
        Controller {
            animations: vec![],
        }
    }

    pub fn add_instance(&mut self, instance: AnimationInstance) {
        self.animations.push(instance);
    }

    pub fn add_time(&mut self, time: f32) {
        for animation in self.animations.iter_mut() {
            animation.animator.add_time(time);
        }
    }

    pub fn update_animators<A: Animation, L: AnimationLibrary<A>>(&mut self, library: &L) -> Result<(), MissingAnimationError> {
        for animation in self.animations.iter_mut() {
            animation.update_frames(library)?
        }

        Ok(())
    }

    pub fn update_pose<A: Animation, L: AnimationLibrary<A>, T: AnimationTarget>(&self, library: &L, targets: &mut [T]) -> Result<(), Box<error::Error>> {
        for animation in &self.animations {
            animation.update_pose(library, targets)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MissingAnimationError {
    animation: usize,
}

impl MissingAnimationError {
    pub fn new(animation: usize) -> MissingAnimationError {
        MissingAnimationError {
            animation,
        }
    }
}

impl fmt::Display for MissingAnimationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing {} animation", self.animation)
    }
}

impl error::Error for MissingAnimationError {}