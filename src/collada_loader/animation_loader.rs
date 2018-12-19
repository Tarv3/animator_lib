use animation::Animation;
use collada;
use math::make_mat4_from_array;
use pose::Pose;
use std::{error, fmt};

// Place Holder Error
#[derive(Debug, Clone, Copy)]
pub struct AnimationLoadError;

impl fmt::Display for AnimationLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to load animation")
    }
}

impl error::Error for AnimationLoadError {
    fn description(&self) -> &str {
        "Failed to load animation"
    }
}

pub fn get_joint_index(
    skeleton: &collada::Skeleton,
    animation: &collada::Animation,
) -> Option<usize> {
    let target = &animation.target.split("/").next()?;

    for (i, joint) in skeleton.joints.iter().enumerate() {
        if joint.name == *target {
            return Some(i);
        }
    }

    None
}

pub fn reorder_animations(
    mut animations: Vec<collada::Animation>,
    skeleton: &collada::Skeleton,
) -> Result<Vec<collada::Animation>, AnimationLoadError> {
    let mut failed = false;
    for (i, animation) in animations.iter().enumerate() {
        println!("Animation {} for \"{}\"", i, animation.target);
    }
    println!("");

    animations.sort_by(|a, b| {
        let a_index = get_joint_index(skeleton, a).unwrap_or_else(|| {
            failed = true;
            0
        });
        let b_index = get_joint_index(skeleton, b).unwrap_or_else(|| {
            failed = true;
            0
        });

        a_index.cmp(&b_index)
    });


    if failed {
        Err(AnimationLoadError)
    } else {
        Ok(animations)
    }
}

pub fn animations_same_sample_times<'a>(
    mut animations: impl Iterator<Item = &'a collada::Animation>,
) -> bool {
    let first_times = match animations.next() {
        Some(animation) => &animation.sample_times,
        None => return true,
    };

    for animation in animations {
        if animation.sample_times != *first_times {
            return false;
        }
    }

    true
}

// Animations must only contain animations for the given skeleton 
pub fn load_animation(animations: Vec<collada::Animation>, bones: usize) -> Result<Animation, AnimationLoadError> {
    if animations.len() != bones {
        return Err(AnimationLoadError);
    }

    if !animations_same_sample_times(animations.iter()) {
        return Err(AnimationLoadError);
    }

    let frames = animations[0].sample_times.len();

    let mut output = Animation::with_capacity(bones, frames);
    let mut keyframe: Vec<Pose> = Vec::with_capacity(bones);

    for i in 0..frames {
        keyframe.clear();
        let sample_time = animations[0].sample_times[i];
        for animation in &animations {
            let matrix = make_mat4_from_array(&animation.sample_poses[i]);
            let pose = Pose::from_matrix(&matrix);
            keyframe.push(pose);
        }

        output.add_frame(keyframe.as_slice(), sample_time);
    }

    Ok(output)
}
