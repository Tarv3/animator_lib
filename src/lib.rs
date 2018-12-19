extern crate nalgebra_glm as glm;
extern crate ordered_float as of;
extern crate collada;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod skeleton;
pub mod pose;
pub mod animation;
pub mod collada_loader;
pub mod math;
pub mod mesh;

#[cfg(test)]
mod glm_test;