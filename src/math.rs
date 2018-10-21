use glm::*;

pub fn minf32(value: f32, max: f32) -> f32 {
    if value < max {
        value
    }
    else {
        max
    }
}

pub fn maxf32(value: f32, min: f32) -> f32 {
    if value > min {
        value
    }
    else {
        min
    }
}

pub fn print_mat4(mat: &Mat4) {
    for row in 0..4 {
        for column in 0..4 {
            print!("{} ", mat[(row, column)]);
        }
        print!("\n");
    }
}

pub fn print_mat3(mat: &Mat3) {
    for row in 0..3 {
        for column in 0..3 {
            print!("{} ", mat[(row, column)]);
        }
        print!("\n");
    }
}


// Assumes that there is no shearing
pub fn separate_rot_scale(m: &Mat3) -> (Vec3, Mat3) {
    let x_axis = column(&m, 0);
        let y_axis = column(&m, 1);
        let z_axis = column(&m, 2);

        let scale = Vec3::new(length(&x_axis), length(&y_axis), length(&z_axis));

        let rot = mat3(
            x_axis.x / scale.x, y_axis.x / scale.y, z_axis.x / scale.z,
            x_axis.y / scale.x, y_axis.y / scale.y, z_axis.y / scale.z,
            x_axis.z / scale.x, y_axis.z / scale.y, z_axis.z / scale.z,
        );

        (scale, rot)
}

#[cfg(test)]
mod tests {
    use glm::*;
    use glm;
    use math;

    #[test]
    fn separate_rot_and_scale() {
        let qpi: f32 = quarter_pi();
        let actual_rot = rotate_x(&identity(), qpi);
        let e_scale = Vec3::new(1.0, 2.0, 1.0);
        let e_rot = mat4_to_mat3(&actual_rot);
        let mat = glm::scale(&actual_rot, &e_scale);
        
        let (scale, rotation) = math::separate_rot_scale(&mat4_to_mat3(&mat));
        println!("rotation = ", );
        math::print_mat3(&rotation);
        println!("scale = {:?}", scale);

        println!("expected rotation = ", );
        math::print_mat3(&e_rot);
        for row in 0..3 {
            for col in 0..3 {
                assert!((e_rot[(row, col)] - rotation[(row, col)]).abs() <= epsilon());
            }
        }
        assert!((scale.x - e_scale.x).abs() <= epsilon());
        assert!((scale.y - e_scale.y).abs() <= epsilon());
        assert!((scale.z - e_scale.z).abs() <= epsilon());
    }
}