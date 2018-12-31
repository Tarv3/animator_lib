use std::error;
use std::fmt;

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

#[derive(Debug, Clone, Copy)]
pub struct MissingFinalPose;

impl fmt::Display for MissingFinalPose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing final pose")
    }
}

impl error::Error for MissingFinalPose {
    fn description(&self) -> &str {
        "Missing final pose"
    }
}