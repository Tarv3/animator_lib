use glm;
use glm::*;
use math::*;

#[test]
fn translation() {
    let mut mat = identity();
    mat[(1, 1)] = 2.0;
    let translation = Vec3::new(1.0, 1.0, 1.0);
    let mat = translate(&mat, &translation);

    let mut expected = identity();
    expected[(1, 1)] = 2.0;
    expected[(0, 3)] = 1.0;
    expected[(1, 3)] = 2.0;
    expected[(2, 3)] = 1.0;
    println!("expected = ");
    print_mat4(&expected);
    println!("mat = ");
    print_mat4(&mat);
    assert!(expected == mat);
}

#[test]
fn scale() {
    let mut mat = identity();
    mat[(1, 3)] = 1.0;
    let vec = Vec3::new(1.0, 2.0, 1.0);
    let mat = glm::scale(&mat, &vec);

    let mut expected = identity();
    expected[(1, 1)] = 2.0;
    expected[(1, 3)] = 1.0;
    println!("expected = ");
    print_mat4(&expected);
    println!("mat = ");
    print_mat4(&mat);
    assert!(expected == mat);
}

