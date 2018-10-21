pub mod animator;

use pose::*;

pub struct Animation {
    keyframes: usize,
    bones: usize,
    poses: Vec<Pose>,
    times: Vec<f32>,
}

impl Animation {
    pub fn from_poses_and_times(bones: usize, poses: &[Pose], times: &[f32]) -> Animation {
        let keyframes = times.len();
        assert!(poses.len() == bones * keyframes);
        let mut times_iter = times.iter();
        let times = match times_iter.next() {
            Some(time) => times_iter.map(|t| *t - time).collect(),
            None => vec![]
        };

        Animation {
            keyframes,
            bones, 
            poses: poses.iter().map(|pose| *pose).collect(),
            times,
        }
    }

    pub fn with_capacity(bones: usize, keyframes: usize) -> Animation {
        Animation {
            keyframes,
            bones,
            poses: Vec::with_capacity(bones * keyframes),
            times: Vec::with_capacity(keyframes),
        }
    }

    pub fn add_frame(&mut self, poses: &[Pose], time: f32) {
        assert!(poses.len() == self.bones);
        let time = match self.times.last() {
            Some(t) => {
                assert!(*t < time);
                time
            }
            None => time
        };

        self.keyframes += 1;
        self.poses.extend(poses);
        self.times.push(time);
    }

    pub fn get_frame_and_time(&self, frame: usize) -> (&[Pose], f32) {
        assert!(frame < self.keyframes);

        let start = frame * self.bones;
        let end = start + self.bones;

        (&self.poses[start..end], self.times[frame])
    }

    pub fn next_frame_time(&self, current_frame: usize) -> Option<f32> {
        let next = current_frame + 1;
        if next >= self.keyframes {
            return None;
        } 

        Some(self.times[next])
    }
}