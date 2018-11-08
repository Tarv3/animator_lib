use animation::Animation;
use math;
use of::OrderedFloat;
use pose::*;

pub enum AniType {
    Looping(f32),
    Once,
}

pub struct Animator {
    time: f32,
    between: (usize, usize),
    ani_type: AniType,
}

impl Animator {
    pub fn new(ani_type: AniType) -> Animator {
        Animator {
            time: 0.0,
            between: (0, 0),
            ani_type,
        }
    }

    pub fn reset_time(&mut self) {
        self.time = 0.0;
    }

    pub fn write_pose_linear(&self, animation: &Animation, buffer: &mut [Pose]) {
        self.write_pose_with_interpolator(animation, buffer, |x| x);
    }

    pub fn write_pose_with_interpolator<F>(&self, animation: &Animation, buffer: &mut [Pose], interpolator: F)
    where
        F: Fn(f32) -> f32,
    {
        let first = animation.get_frame_and_time(self.between.0);
        let second = animation.get_frame_and_time(self.between.1);

        if self.between.0 == animation.times.len() - 1 {
            match self.ani_type {
                AniType::Looping(duration) => {
                    let interp = (self.time - first.1) / duration;
                    interpolate_poses(&first.0, &second.0, buffer, interpolator(interp));
                    return;
                }
                AniType::Once => {
                    write_poses(first.0, buffer);
                    return;
                }
            }
        }

        let time = second.1 - first.1;
        let interp = (self.time - first.1) / time;
        interpolate_poses(&first.0, &second.0, buffer, interpolator(interp));
    }

    // Adds time to the animation either positive or negative time and then updates
    // the between value.
    pub fn add_time(&mut self, animation: &Animation, time: f32) {
        assert!(animation.keyframes > 0);

        let last = *animation.times.last().unwrap();
        let last: f32 = last.into();

        let mut new_time = self.time + time;

        match self.ani_type {
            AniType::Looping(duration) => {
                new_time = math::time_loop(new_time, 0.0, last + duration)
            }
            _ => new_time = math::clampf32(new_time, 0.0, last),
        }

        self.time = new_time;
        self.update_between(animation, time.is_sign_positive());
    }

    // Binary searches for self.time in animation.times and uses that index to
    // set self.between
    pub fn update_between_binary_search(&mut self, animation: &Animation) {
        let first = match animation
            .times
            .binary_search_by(|num| num.cmp(&OrderedFloat(self.time)))
        {
            Ok(index) => index,
            Err(index) => index - 1,
        };

        let second = match first == animation.times.len() - 1 {
            true => 0,
            false => first + 1,
        };

        self.between = (first, second);
    }

    // Guesses that the new between will be near the old between so it iterates through
    // the times until the between value is correct
    fn update_between(&mut self, animation: &Animation, positive_dir: bool) {
        let len = animation.times.len();
        for _ in 0..len {
            if !self.between_needs_update(animation) {
                break;
            }
            if positive_dir {
                self.between.0 = self.between.1;
                self.between.1 = (self.between.1 + 1) % len;
            } else {
                self.between.1 = self.between.0;
                if self.between.0 == 0 {
                    self.between.0 = len - 1;
                } else {
                    self.between.0 -= 1;
                }
            }
        }
    }

    // Returns if self.time is between self.between
    fn between_needs_update(&self, animation: &Animation) -> bool {
        let first = animation.times[self.between.0].into_inner();
        let second;
        if self.between.0 == animation.times.len() - 1 {
            second = match self.ani_type {
                AniType::Looping(duration) => first + duration,
                _ => first,
            };
        } else {
            second = animation.times[self.between.1].into_inner();
        }

        self.time < first || self.time > second
    }

    pub fn has_finished(&self, animation: &Animation) -> bool {
        assert!(animation.keyframes > 0);
        let last = animation.times.last().unwrap();

        match self.ani_type {
            AniType::Once => last.into_inner() == self.time,
            AniType::Looping(_) => false,
        }
    }
}
