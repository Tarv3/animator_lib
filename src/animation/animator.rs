use pose::*;
use animation::Animation;

pub struct Animator {
    animation: Animation,
    time: f32,
    between: (usize, usize),
}

impl Animator {
    pub fn write_current_pose(&self, buffer: &mut [Pose]) {
        let first = self.animation.get_frame_and_time(self.between.0);
        let second = self.animation.get_frame_and_time(self.between.1);
        let interp = self.time - first.1;

        interpolate_poses(&first.0, &second.0, buffer, interp);
    }
}