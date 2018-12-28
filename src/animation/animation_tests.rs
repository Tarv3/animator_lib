use super::{traits::*, animator2::Animator};
use pose::*;

struct AnimationTest {
    sample_times: Vec<f32>,
    poses: Vec<Pose>,
}

impl Animation for AnimationTest {
    fn frames(&self) -> usize {
        self.sample_times.len()
    }

    fn sample_times(&self) -> &[f32] {
        self.sample_times.as_slice()
    }

    fn get_frame(&self, frame: usize) -> Option<(&[Pose], f32)> {
        if frame < self.sample_times.len() {
            Some((&self.poses[frame..frame + 1], self.sample_times[frame]))
        }
        else {
            None
        }
    }

    fn get_targets<'a>(&'a self) -> Targets<'a> {
        Targets::InOrder
    }
}

#[test]
fn animator_loop_test() {
    let sample_times = vec![0.0, 1.0, 2.0, 3.0];
    let mut animator = Animator::new();
    animator.loop_time = Some(1.0);
    animator.time = 3.5;

    animator.update_frames(sample_times.as_slice());

    println!("{:?}", animator );

    assert!(animator.current_frame() == (3, 3.0));
    assert!(animator.next_frame() == (0, 0.0));
}

#[test]
fn animator_forward_test() {
    let sample_times = vec![0.0, 1.0, 2.0, 3.0];
    let mut animator = Animator::new();
    animator.loop_time = Some(1.0);
    animator.time = 1.5;

    animator.update_frames(sample_times.as_slice());

    println!("{:?}", animator );

    assert!(animator.current_frame() == (1, 1.0));
    assert!(animator.next_frame() == (2, 2.0));
}

#[test]
fn animator_backward_test() {
    let sample_times = vec![0.0, 1.0, 2.0, 3.0];
    let mut animator = Animator::new();
    animator.loop_time = Some(1.0);
    animator.time = 1.5;
    animator.forward_time = false;

    animator.update_frames(sample_times.as_slice());

    println!("{:?}", animator );

    assert!(animator.current_frame() == (2, 2.0));
    assert!(animator.next_frame() == (1, 1.0));
}

#[test]
fn animator_outside_test() {
    let sample_times = vec![0.0, 1.0, 2.0, 3.0];
    let mut animator = Animator::new();
    animator.loop_time = Some(5.0);
    animator.time = 1.5;
    animator.forward_time = true;

    animator.update_frames(sample_times.as_slice());

    println!("{:?}", animator );

    assert!(animator.current_frame() == (1, 1.0));
    assert!(animator.next_frame() == (2, 2.0));
}