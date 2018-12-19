use collada;
use skeleton::{Skeleton, bone::Bone};
use pose::Pose;
use math::make_mat4_from_array;

pub fn load_skeleton(skeleton: &collada::Skeleton) -> Skeleton {
    let mut output = Skeleton::with_capacity(skeleton.joints.len());

    for i in 0..skeleton.joints.len() {
        let joint = &skeleton.joints[i];
        let default_pose = make_mat4_from_array(&skeleton.bind_poses[i]);
        let default_pose = Pose::from_matrix(&default_pose);

        let parent = match joint.is_root() {
            true => None,
            false => Some(joint.parent_index as usize),
        };

        // let inv_bindpose = make_mat4_from_array(&joint.inverse_bind_pose);
        // let inv_bindpose = Pose::from_matrix(&inv_bindpose);
        let bone = Bone::new(default_pose, parent);
        output.add_bone(bone);
    }
    output.build_inv_bindposes();
    output
}