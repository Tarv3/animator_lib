use pose::*;
use math;
use animation::traits::{Animation, AnimationTarget, Targets};
use std::{cmp::PartialOrd, fmt, error};
use glm::*;

#[derive(Debug, Clone, Copy)]
pub struct Animator {
    pub time: f32,
    loop_time: Option<f32>,
    forward_time: bool,
    current_frame: (usize, f32),
    next_frame: (usize, f32),
}

impl Animator {
    pub fn new() -> Animator {
        Animator {
            time: 0.0,
            loop_time: None,
            forward_time: true,
            current_frame: (0, 0.0),
            next_frame: (0, 0.0),
        }
    }

    pub fn set_loop_time(&mut self, time: f32) {
        self.loop_time = Some(time);
    }

    pub fn remove_loop(&mut self) {
        self.loop_time = None;
    }

    pub fn set_time_direction(&mut self, forwards: bool) {
        self.forward_time = forwards;
    }

    pub fn reverse(&mut self) {
        self.forward_time = !self.forward_time;
    }

    pub fn add_time(&mut self, time: f32) {
        if self.forward_time {
            self.time += time;
        }
        else {
            self.time -= time;
        }
    }

    pub fn current_frame(&self) -> (usize, f32) {
        self.current_frame
    }
    
    pub fn next_frame(&self) -> (usize, f32) {
        self.next_frame
    }
    
    pub fn update_frames(&mut self, sample_times: &[f32]) {
        let len = sample_times.len();
        assert!(len > 0);

        if self.time >= self.current_frame.1 && self.time < self.next_frame.1 {
            return;
        }

        let (last_ind, last_time) = match len > 0 {
            true => (len - 1, sample_times[len - 1]),
            false => (0, 0.0),
        };
        
        if self.loop_time.is_some() {
            let loop_time = self.loop_time.unwrap();
            let duration = loop_time + last_time;
            self.time = math::time_loop(self.time, 0.0, duration);
         
            if self.time >= last_time && self.time < duration {
                self.current_frame = (last_ind, last_time);
                self.next_frame = (0, 0.0);
                return;
            }
        }
        else if self.time >= last_time && self.forward_time {
            self.current_frame = (last_ind, last_time);
            self.next_frame = (last_ind, last_time);
            return;
        }
        else if self.time <= 0.0 {
            self.current_frame = (0, 0.0);
            self.next_frame = (0, 0.0);
            return;
        }

        if self.forward_time {
            if self.time < self.current_frame.1 {
                let index = match sample_times[..self.current_frame.0].binary_search_by(|x| x.partial_cmp(&self.time).unwrap()) {
                    Ok(index) => index,
                    Err(index) => index - 1,
                };

                self.current_frame = (index, sample_times[index]);
                self.next_frame = (index + 1, sample_times[index + 1]);
                return;
            }

            self.current_frame = self.next_frame;
            let next = self.next_frame.0 + 1;
            self.next_frame = (next, sample_times[next]);
            

            while !(self.time >= self.current_frame.1 && self.time < self.next_frame.1) {
                self.current_frame = self.next_frame;
                let next = self.next_frame.0 + 1;
                self.next_frame = (next, sample_times[next]);
            }

            return;
        }
        else {
            if self.time > self.next_frame.1 {
                let index = match sample_times[self.next_frame.0..].binary_search_by(|x| x.partial_cmp(&self.time).unwrap()) {
                    Ok(index) => index,
                    Err(index) => index,
                };

                self.next_frame = (index, sample_times[index]);
                self.current_frame = (index - 1, sample_times[index - 1]);
                return;
            }

            self.next_frame = self.current_frame;
            let prev = match self.current_frame.0 == 0 {
                false => self.current_frame.0 - 1,
                true => last_ind,
            };

            self.current_frame = (prev, sample_times[prev]);
            while !(self.time >= self.current_frame.1 && self.time < self.next_frame.1) {
                self.next_frame = self.current_frame;
                let prev = self.current_frame.0 - 1;
                self.current_frame = (prev, sample_times[prev]);;
            }
        }

    }   

    pub fn manipulate_pose<A, T, F>(&self, animation: &A, targets: &mut [T], mut function: F) -> Result<(), MissingFrameError> 
    where
        A: Animation,
        T: AnimationTarget + Sized,
        F: FnMut(Pose, Pose, f32, &mut T),
    {
        let frames = animation.frames();
        if frames == 0 {
            return Ok(());
        }

        let last = frames - 1;
        let (cposes, ctime) = animation.get_frame(self.current_frame.0).ok_or(MissingFrameError::new(self.current_frame.0))?;
        let (nposes, ntime)= animation.get_frame(self.next_frame.0).ok_or(MissingFrameError::new(self.next_frame.0))?;

        let interpolate;

        if self.current_frame.0 == last {
            match self.loop_time {
                Some(duration) => interpolate = (self.time - ctime) / duration,
                None => interpolate = 0.0,
            }
        }
        else {
            interpolate = (self.time - ctime) / (ntime - ctime);
        }

        match animation.get_targets() {
            Targets::Specified(array) => {
                for (i, target) in array.iter().enumerate() {
                    function(cposes[i], nposes[i], interpolate, &mut targets[*target]);
                }
            }
            Targets::InOrder => {
                for (i, target) in targets.iter_mut().enumerate() {
                    function(cposes[i], nposes[i], interpolate, target);
                }
            }
        }

        Ok(())
    }

    pub fn write_pose<A, T>(&self, animation: &A, targets: &mut [T]) -> Result<(), MissingFrameError>
    where
        A: Animation,
        T: AnimationTarget,
    {
        self.manipulate_pose(animation, targets, |a, b, interpolate, target| {
            let pose = pose_interp(&a, &b, interpolate);
            target.set_pose(pose);
        })
    }

    pub fn add_pose<A, T>(&self, animation: &A, targets: &mut [T]) -> Result<(), MissingFrameError>
    where
        A: Animation,
        T: AnimationTarget + Sized,
    {
        self.manipulate_pose(animation, targets, |a, b, interpolate, target| {
            let pose = pose_interp(&a, &b, interpolate);
            target.add_pose(pose);
        })
    }

    pub fn add_rotations<A, T>(&self, animation: &A, targets: &mut [T]) -> Result<(), MissingFrameError> 
    where
        A: Animation,
        T: AnimationTarget + Sized,
    {
        self.manipulate_pose(animation, targets, |a, b, interpolate, target| {
            let rotation;
            let a_rot = a.rotation;
            let b_rot = b.rotation;

            if dot(&a_rot.coords, &b_rot.coords) < 0.0 {
                rotation = quat_slerp(&-a_rot, &b_rot, interpolate);
            } else {
                rotation = quat_slerp(&a_rot, &b_rot, interpolate);
            }

            target.add_rotation(rotation);
        })
    }

    pub fn write_rotations<A, T>(&self, animation: &A, targets: &mut [T]) -> Result<(), MissingFrameError> 
    where
        A: Animation,
        T: AnimationTarget + Sized,
    {
        self.manipulate_pose(animation, targets, |a, b, interpolate, target| {
            let rotation;
            let a_rot = a.rotation;
            let b_rot = b.rotation;

            if dot(&a_rot.coords, &b_rot.coords) < 0.0 {
                rotation = quat_slerp(&-a_rot, &b_rot, interpolate);
            } else {
                rotation = quat_slerp(&a_rot, &b_rot, interpolate);
            }

            target.get_pose_mut().rotation = rotation;
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MissingFrameError {
    frame: usize,
}

impl MissingFrameError {
    pub fn new(frame: usize) -> MissingFrameError {
        MissingFrameError {
            frame,
        }
    }
}

impl fmt::Display for MissingFrameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Animation missing {} frame", self.frame)
    }
}

impl error::Error for MissingFrameError {}