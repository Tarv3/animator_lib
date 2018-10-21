use pose::Pose;
use std::error;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Bone {
    pub parent: Option<usize>,
    pub pose: Pose,

    pub final_pose: Option<Pose>,
    pub inv_pose: Option<Pose>,
}

impl Bone {
    pub fn new(pose: Pose, parent: Option<usize>) -> Bone {
        Bone {
            parent,
            pose,
            final_pose: None,
            inv_pose: None,
        }
    }

    pub fn with_inv_pose(pose: Pose, inv_pose: Pose, parent: Option<usize>) -> Bone {
        Bone {
            parent,
            pose,
            final_pose: None,
            inv_pose: Some(inv_pose),
        }
    }

    pub fn reset(&mut self) {
        self.final_pose = None;
    }

    pub fn get_inv_pose(&self) -> &Option<Pose> {
        &self.inv_pose
    }

    // Recursively builds the bindpose pose for self and any parent bone
    // Requires there to be a connection to a root bone
    pub fn build_inv_pose(&mut self, bones: &mut [Bone]) -> Pose {
        if let Some(pose) = self.inv_pose {
            return pose;
        }
        let inv_pose = self.pose.inverse();
        let pose = match self.parent {
            Some(id) => {
                if let Some(pose) = bones[id].inv_pose {
                    inv_pose * pose
                } else {
                    let mut parent = bones[id];
                    let pose = parent.build_inv_pose(bones);
                    bones[id] = parent;
                    inv_pose * pose
                }
            }
            None => inv_pose,
        };
        self.inv_pose = Some(pose);
        pose
    }

    // Recursively builds the pose matrices for self and any parent bone of self
    pub fn build_pose(&mut self, bones: &mut Vec<Bone>) -> Result<Pose, MissingInvBindpose> {
        if self.inv_pose.is_none() {
            return Err(MissingInvBindpose);
        }
        if let Some(pose) = self.final_pose {
            return Ok(pose);
        }

        let pose = self.pose;
        let pose = match self.parent {
            Some(id) => {
                if bones[id].final_pose.is_none() {
                    let mut parent = bones[id];
                    let p_pose = parent.build_pose(bones)?;
                    bones[id] = parent;
                    p_pose * pose
                } else {
                    let p_pose = bones[id].final_pose.unwrap();
                    p_pose * pose
                }
            }
            None => pose,
        };
        let inv_pose = self.inv_pose.unwrap();
        let final_pose = pose * inv_pose;
        self.final_pose = Some(final_pose);

        Ok(final_pose)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MissingInvBindpose;

impl fmt::Display for MissingInvBindpose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing inverse bind pose matrix")
    }
}

impl error::Error for MissingInvBindpose {
    fn description(&self) -> &str {
        "Missing inverse bind pose matrix"
    }
}