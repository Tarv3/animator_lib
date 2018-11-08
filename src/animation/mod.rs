pub mod animator;

use of::OrderedFloat;
use pose::*;

#[derive(Clone, Debug)]
pub struct Animation {
    pub keyframes: usize,
    pub bones: usize,
    pub poses: Vec<Pose>,
    pub times: Vec<OrderedFloat<f32>>,
}

impl Animation {
    // Builds animation with the times moved such that time[0] = 0
    pub fn from_poses_and_times(bones: usize, poses: &[Pose], times: &[f32]) -> Animation {
        let keyframes = times.len();
        assert!(poses.len() == bones * keyframes);

        let mut times_iter = times.iter();
        let mut times: Vec<OrderedFloat<f32>> = Vec::with_capacity(times.len());
        match times_iter.next() {
            Some(time) => {
                times.push(OrderedFloat(*time));   
                times.extend(times_iter.map(|t| OrderedFloat(t - time)));
            },
            None => (),
        }

        Animation {
            keyframes,
            bones,
            poses: poses.iter().map(|pose| *pose).collect(),
            times,
        }
    }

    pub fn with_capacity(bones: usize, keyframes: usize) -> Animation {
        Animation {
            keyframes: 0,
            bones,
            poses: Vec::with_capacity(bones * keyframes),
            times: Vec::with_capacity(keyframes),
        }
    }

    pub fn add_frame(&mut self, poses: &[Pose], time: impl Into<OrderedFloat<f32>>) {
        assert!(poses.len() == self.bones);
        let time = time.into();
        let time = match self.times.last() {
            Some(t) => {
                assert!(*t < time);
                time
            }
            None => OrderedFloat(0.0),
        };

        self.keyframes += 1;
        self.poses.extend(poses);
        self.times.push(time);
    }

    pub fn get_frame_and_time(&self, frame: usize) -> (&[Pose], f32) {
        assert!(frame < self.keyframes);

        let start = frame * self.bones;
        let end = start + self.bones;

        (&self.poses[start..end], self.times[frame].into())
    }

    pub fn next_frame_time(&self, current_frame: usize) -> Option<f32> {
        let next = current_frame + 1;
        if next >= self.keyframes {
            return None;
        }

        Some(self.times[next].into())
    }
}
