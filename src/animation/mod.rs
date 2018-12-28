pub mod animator;
pub mod traits;
pub mod library;
pub mod controller;
#[cfg(test)]
mod animation_tests;

use pose::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::path::Path;
use std::error::Error;
use serde_json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Animation {
    pub keyframes: usize,
    pub bones: usize,
    pub poses: Vec<Pose>,
    pub times: Vec<f32>,
    pub targets: Option<Vec<usize>>,
}

impl Animation {
    // Builds animation with the times moved such that time[0] = 0
    pub fn from_poses_and_times(bones: usize, poses: &[Pose], times: &[f32], targets: Option<Vec<usize>>) -> Animation {
        let keyframes = times.len();
        assert!(poses.len() == bones * keyframes);

        let mut times_iter = times.iter();
        let mut times: Vec<f32> = Vec::with_capacity(times.len());
        match times_iter.next() {
            Some(time) => {
                times.push(*time);   
                times.extend(times_iter.map(|t| t - time));
            },
            None => (),
        }

        Animation {
            keyframes,
            bones,
            poses: poses.iter().map(|pose| *pose).collect(),
            times,
            targets,
        }
    }

    pub fn with_capacity(bones: usize, keyframes: usize, targets: Option<Vec<usize>>) -> Animation {
        Animation {
            keyframes: 0,
            bones,
            poses: Vec::with_capacity(bones * keyframes),
            times: Vec::with_capacity(keyframes),
            targets,
        }
    }

    pub fn new(bones: usize, targets: Option<Vec<usize>>) -> Animation {
        Animation {
            keyframes: 0,
            bones,
            poses: vec![],
            times: vec![],
            targets,
        }
    }

    pub fn add_frame(&mut self, poses: &[Pose], time: impl Into<f32>) {
        assert!(poses.len() == self.bones);
        let time = time.into();
        let time = match self.times.last() {
            Some(t) => {
                assert!(*t < time);
                time
            }
            None => 0.0,
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

    pub fn load_from(path: impl AsRef<Path>) -> Result<Animation, Box<Error>>{
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let animation = serde_json::from_reader(reader)?;
        Ok(animation)
    }   

    pub fn save_to(&self, path: impl AsRef<Path>) -> Result<(), Box<Error>> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer(writer, &self)?;
        Ok(())
    }
}


impl traits::Animation for Animation {
    fn sample_times(&self) -> &[f32] {
        self.times.as_slice()
    }

    fn get_frame(&self, frame: usize) -> Option<(&[Pose], f32)> {
        if frame >= self.times.len() {
            return None;
        }

        Some(self.get_frame_and_time(frame))
    }

    fn get_targets<'a>(&'a self) -> traits::Targets<'a> {
        match &self.targets {
            Some(target) => traits::Targets::Specified(target),
            None => traits::Targets::InOrder,
        }
    }
}