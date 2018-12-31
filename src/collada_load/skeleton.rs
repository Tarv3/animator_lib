use collada_parser::collada::{self, Skin};
use skeleton::{Skeleton, error::*};
use math::*;
use pose::*;

pub fn load_skeleton(skeleton: &collada::Skeleton) -> Skeleton {
    let mut tree = Vec::with_capacity(skeleton.nodes.len());
    let mut poses = Vec::with_capacity(skeleton.nodes.len());

    for node in &skeleton.nodes {
        let parent = node.parent;
        let matrix = mat4_from_matrix4(&node.default_trans);
        let pose = Pose::from_matrix(&matrix);

        tree.push(parent);
        poses.push(pose);
    }

    Skeleton::from_tree_pose(tree, poses)
}

pub fn set_bind_poses_skeleton(skeleton: &mut Skeleton, skin: &Skin) -> Result<(), MissingInvBindpose> {
    skeleton.set_inv_bind_pose_iter(skin.bind_poses.iter().map(|x| {
        let matrix = mat4_from_matrix4(x);
        Pose::from_matrix(&matrix)
    }))
} 