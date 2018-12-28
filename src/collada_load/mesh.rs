use collada_parser::collada::{Mesh, Skin, mesh::primitive_elements::Shape, skin::JointWeight};
use collada_parser::math::{Vector3, Vector2};
use mesh;
use std::collections::{HashMap, hash_map::Entry};
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Copy, Clone, Debug)]
pub struct MeshLoadError;

impl Display for MeshLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to load mesh")
    }
}

impl Error for MeshLoadError {}

pub trait VertexFromParts: Sized {
    fn from_parts(vertex: Vector3, tvertex: Option<Vector2>, normal: Option<Vector3>, weights: Option<&[JointWeight]>) -> Option<Self>;
}

pub fn load_mesh<V: VertexFromParts + Copy>(mesh: &Mesh, skin: Option<&Skin>) -> Result<mesh::Mesh<V>, MeshLoadError> {
    let mut vertices = vec![];
    let mut shapes = vec![];
    let mut indices: HashMap<(usize, Option<usize>, Option<usize>), usize> = HashMap::new();
    let mut current_index = 0;

    {
        let mut insert_update = |(vertex, tex, normal): (usize, Option<usize>, Option<usize>)| -> Result<usize, MeshLoadError> {
            let index = current_index;
            current_index += 1;

            let vert = mesh.vertices[vertex];
            let tex = tex.map(|x| mesh.tex_coords[x]);
            let normal = normal.map(|x| mesh.normals[x]);
            let weights = skin.map(|x| x.vertex_weights[vertex].as_slice());

            let vertex = match V::from_parts(vert, tex, normal, weights) {
                Some(vertex) => vertex,
                None => {
                    return Err(MeshLoadError)
                }
            };
            vertices.push(vertex);

            Ok(index)
        };

        for shape in &mesh.shapes {
            match shape {
                Shape::Line(a, b) => {
                    let a = match indices.entry((a.0, a.1, a.2)) {
                        Entry::Vacant(entry) => *entry.insert(insert_update((a.0, a.1, a.2))?),
                        Entry::Occupied(entry) => *entry.get()
                    };
                    let b = match indices.entry((b.0, b.1, b.2)) {
                        Entry::Vacant(entry) => *entry.insert(insert_update((b.0, b.1, b.2))?),
                        Entry::Occupied(entry) => *entry.get()
                    };

                    shapes.push(mesh::Shape::Line(a, b));
                },
                Shape::Triangle(a, b, c) => {
                    let a = match indices.entry((a.0, a.1, a.2)) {
                        Entry::Vacant(entry) => *entry.insert(insert_update((a.0, a.1, a.2))?),
                        Entry::Occupied(entry) => *entry.get()
                    };
                    let b = match indices.entry((b.0, b.1, b.2)) {
                        Entry::Vacant(entry) => *entry.insert(insert_update((b.0, b.1, b.2))?),
                        Entry::Occupied(entry) => *entry.get()
                    };
                    let c = match indices.entry((c.0, c.1, c.2)) {
                        Entry::Vacant(entry) => *entry.insert(insert_update((c.0, c.1, c.2))?),
                        Entry::Occupied(entry) => *entry.get()
                    };

                    shapes.push(mesh::Shape::Triangle(a, b, c));
                },
                _ => {}
            }
        }
    }
    Ok(mesh::Mesh {
        vertices,
        shapes,
    })
} 