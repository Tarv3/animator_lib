use collada::*;
use mesh;
use std::collections::{HashMap, hash_map::Entry};
use std::error::Error;
use std::fmt::Display;
use std::fmt;

pub type TexVert = TVertex;
pub type Vert = Vertex;
pub type Weights = JointWeights;

fn get_parts(index: VTNIndex, obj: &Object) -> (Vertex, Option<TVertex>, Option<Vertex>, Option<JointWeights>) {
    let vertex = obj.vertices[index.0];
    let texture = index.1.map(|index| obj.tex_vertices[index]);
    let normal = index.2.map(|index| obj.normals[index]);
    let joint = match obj.joint_weights.len() == obj.vertices.len() {
        true => Some(obj.joint_weights[index.0]),
        false => None,
    };

    (vertex, texture, normal, joint)
}

pub trait VertexFromParts: Sized {
    fn from_parts(vertex: Vertex, tvertex: Option<TVertex>, normal: Option<Vertex>, weights: Option<JointWeights>) -> Option<Self>;
}

pub fn load_mesh<T: VertexFromParts + Copy>(obj: &Object) -> Result<mesh::Mesh<T>, MeshLoadError> {
    let mut vertices = vec![];
    let mut shapes = vec![];
    let mut indices: HashMap<(usize, Option<usize>, Option<usize>), usize> = HashMap::new();
    let mut current_index = 0;

    {
        let mut insert_update = |vtn_index: VTNIndex| -> Result<usize, MeshLoadError> {
            let index = current_index;
            current_index += 1;
    
            let (vertex, tvertex, normal, weights) = get_parts(vtn_index, obj);
            let vertex = match T::from_parts(vertex, tvertex, normal, weights) {
                Some(vertex) => vertex,
                None => return Err(MeshLoadError),
            };
    
            vertices.push(vertex);
    
            Ok(index)
        };
        
        for geometry in &obj.geometry {
            for shape in &geometry.shapes {
                match shape {
                    Shape::Point(a) => {
                        let a = match indices.entry(*a) {
                            Entry::Vacant(entry) => *entry.insert(insert_update(*a)?),
                            Entry::Occupied(entry) => *entry.get()
                        };
    
                        shapes.push(mesh::Shape::Point(a));
                    },
                    Shape::Line(a, b) => {
                        let a = match indices.entry(*a) {
                            Entry::Vacant(entry) => *entry.insert(insert_update(*a)?),
                            Entry::Occupied(entry) => *entry.get()
                        };
                        let b = match indices.entry(*b) {
                            Entry::Vacant(entry) => *entry.insert(insert_update(*b)?),
                            Entry::Occupied(entry) => *entry.get()
                        };
    
                        shapes.push(mesh::Shape::Line(a, b));
                    },
                    Shape::Triangle(a, b, c) => {
                        let a = match indices.entry(*a) {
                            Entry::Vacant(entry) => *entry.insert(insert_update(*a)?),
                            Entry::Occupied(entry) => *entry.get()
                        };
                        let b = match indices.entry(*b) {
                            Entry::Vacant(entry) => *entry.insert(insert_update(*b)?),
                            Entry::Occupied(entry) => *entry.get()
                        };
                        let c = match indices.entry(*c) {
                            Entry::Vacant(entry) => *entry.insert(insert_update(*c)?),
                            Entry::Occupied(entry) => *entry.get()
                        };
    
                        shapes.push(mesh::Shape::Triangle(a, b, c));
                    },
                }
            }
        }
    }
    Ok(mesh::Mesh::new(vertices, shapes))
}

#[derive(Copy, Clone, Debug)]
pub struct MeshLoadError;

impl Display for MeshLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to load mesh")
    }
}

impl Error for MeshLoadError {}