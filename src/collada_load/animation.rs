use animation;
use collada_parser::collada::{Skeleton, Animation};
use std::error::Error;
use std::fmt::{self, Display};
use pose::*;
use math::*;

#[derive(Copy, Clone, Debug)]
pub struct AnimationLoadError;

impl Display for AnimationLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to load animation")
    }
}

impl Error for AnimationLoadError {}

pub fn load_animation(skeleton: &Skeleton, ani: &[Animation]) -> Result<animation::Animation, AnimationLoadError> {
    let mut sample_times = None;
    let mut animations = vec![];

    for animation in skeleton.animations(ani) {
        let animation = animation.ok_or(AnimationLoadError)?;

        if sample_times.is_none() {
            sample_times = Some(&animation.sample_times);
        }
        else if Some(&animation.sample_times) != sample_times {
            return Err(AnimationLoadError);
        }

        animations.push(&animation.transformations);
    }

    let keyframes = match sample_times {
        Some(times) => times.len(),
        None => return Err(AnimationLoadError), 
    };

    let bones = skeleton.nodes.len();
    let mut animation = animation::Animation::with_capacity(bones, keyframes);

    let mut poses = Vec::with_capacity(bones); 
    let sample_times = sample_times.unwrap();
    for i in 0..keyframes {
        poses.clear();
        let time = sample_times[i];

        for j in 0..bones {
            let matrix = mat4_from_matrix4(&animations[j][i]);
            poses.push(Pose::from_matrix(&matrix));
        }

        animation.add_frame(poses.as_ref(), time);
    }

    Ok(animation)
}