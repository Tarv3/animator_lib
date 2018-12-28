use collada_parser::collada::{self, Skin};
use skeleton::{Skeleton, bone::Bone};
use math::*;
use pose::*;

pub fn load_skeleton(skeleton: &collada::Skeleton) -> Skeleton {
    let mut bones = Vec::with_capacity(skeleton.nodes.len());

    for node in &skeleton.nodes {
        let parent = node.parent;
        let matrix = mat4_from_matrix4(&node.default_trans);
        let pose = Pose::from_matrix(&matrix);

        let bone = Bone::new(pose, parent);
        bones.push(bone);
    }

    Skeleton::from_bones(bones)
}

pub fn set_bind_poses(skeleton: &mut Skeleton, skin: &Skin) {
    for (i, bone) in skeleton.bones_mut_ref().iter_mut().enumerate() {
        let matrix = mat4_from_matrix4(&skin.bind_poses[i]);
        let pose = Pose::from_matrix(&matrix);

        bone.inv_pose = Some(pose);
    }
} 