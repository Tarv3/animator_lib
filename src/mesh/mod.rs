// use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Shape {
    Point(usize),
    Line(usize, usize),
    Triangle(usize, usize, usize),
}

impl Shape {
    pub fn is_triangle(&self) -> bool {
        match self {
            Shape::Triangle(_, _, _) => true,
            _ => false,
        }
    }

    pub fn is_line(&self) -> bool {
        match self {
            Shape::Line(_, _) => true,
            _ => false,
        }
    }

    pub fn is_point(&self) -> bool {
        match self {
            Shape::Point(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mesh<VERTEX: Copy> {
    pub vertices: Vec<VERTEX>,
    pub shapes: Vec<Shape>
}

impl<VERTEX: Copy> Mesh<VERTEX> {
    pub fn new(vertices: Vec<VERTEX>, shapes: Vec<Shape>) -> Mesh<VERTEX> {
        Mesh {
            vertices,
            shapes
        }
    }

    pub fn only_triangles(&self) -> bool {
        for shape in &self.shapes {
            if !shape.is_triangle() {
                return false;
            }
        }
        true
    }

    pub fn flatten_indices(&self) -> Vec<usize> {
        let mut indices = vec![];
        for shape in &self.shapes {
            match shape {
                Shape::Triangle(a, b, c) => {
                    indices.push(*a);
                    indices.push(*b);
                    indices.push(*c);
                }
                Shape::Line(a, b) => {
                    indices.push(*a);
                    indices.push(*b);
                }
                Shape::Point(a) => indices.push(*a)
            }
        }

        indices
    }

    pub fn get_triangles(&self) -> SingleShapeMesh<VERTEX> {
        if self.only_triangles() {
            let indices = self.flatten_indices();
            let vertices = self.vertices.clone();

            return SingleShapeMesh::new(vertices, indices);
        }

        let mut vertices = vec![];
        let mut indices = vec![];
        let mut used_indices = BTreeMap::new();

        let mut current_index = 0;

        for shape in &self.shapes {
            let triangle = match shape {
                Shape::Triangle(a, b, c) => (a, b, c),
                _ => continue,
            };

            let triangle_indices = (
                *used_indices.entry(triangle.0).or_insert_with(|| {
                    let index = current_index;
                    current_index += 1;

                    vertices.push(self.vertices[index]);
                    index
                }),
                *used_indices.entry(triangle.1).or_insert_with(|| {
                    let index = current_index;
                    current_index += 1;

                    vertices.push(self.vertices[index]);
                    index
                }),
                *used_indices.entry(triangle.2).or_insert_with(|| {
                    let index = current_index;
                    current_index += 1;

                    vertices.push(self.vertices[index]);
                    index
                }),
            );

            indices.push(triangle_indices.0);
            indices.push(triangle_indices.1);
            indices.push(triangle_indices.2);

        }

        SingleShapeMesh::new(vertices, indices)
    }

    pub fn get_lines(&self) -> SingleShapeMesh<VERTEX> {
        let mut vertices = vec![];
        let mut indices = vec![];
        let mut used_indices = BTreeMap::new();

        let mut current_index = 0;

        for shape in &self.shapes {
            let line = match shape {
                Shape::Line(a, b) => (a, b),
                _ => continue,
            };

            let line_indices = (
                *used_indices.entry(line.0).or_insert_with(|| {
                    let index = current_index;
                    current_index += 1;

                    vertices.push(self.vertices[index]);
                    index
                }),
                *used_indices.entry(line.1).or_insert_with(|| {
                    let index = current_index;
                    current_index += 1;

                    vertices.push(self.vertices[index]);
                    index
                }),
            );

            indices.push(line_indices.0);
            indices.push(line_indices.1);
        }
        SingleShapeMesh::new(vertices, indices)

    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SingleShapeMesh<VERTEX: Copy> {
    pub vertices: Vec<VERTEX>,
    pub indices: Vec<usize>
}

impl<VERTEX: Copy> SingleShapeMesh<VERTEX> {
    pub fn new(vertices: Vec<VERTEX>, indices: Vec<usize>) -> Self {
        Self {
            vertices,
            indices
        }
    }
}