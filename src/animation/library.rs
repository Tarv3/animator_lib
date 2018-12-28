use super::traits::{Animation, AnimationLibrary};

pub struct AniLibrary<A: Animation> {
    animations: Vec<A>,
}

impl<A: Animation> AniLibrary<A> {
    pub fn new() -> AniLibrary<A> {
        AniLibrary {
            animations: vec![],
        }
    }

    pub fn add_animation(&mut self, animation: A) {
        self.animations.push(animation);
    }
}

impl<A: Animation> AnimationLibrary<A> for AniLibrary<A> {
    fn get_animation(&self, index: usize) -> Option<&A> {
        if index >= self.animations.len() {
            None
        }
        else {
            Some(&self.animations[index])
        }
    }
}