extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

trait WaveformType {
    fn new(obj_string: String) -> Self;

    const DESCRIPTOR: &'static str;
    const REGEX: &'static str;
}

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    const ZERO: Vertex = Vertex {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

impl WaveformType for Vertex {
    fn new(vertex: String) -> Vertex {
        lazy_static! {
            static ref TYPE_REGEX: Regex = Regex::new(Vertex::REGEX).unwrap();
        }

        if let Some(captured_vertex) = TYPE_REGEX.captures(&vertex)  {
            return Vertex {
                x: captured_vertex["x"].parse().unwrap(),
                y: captured_vertex["y"].parse().unwrap(),
                z: captured_vertex["z"].parse().unwrap(),
            }
        } else {
            println!("ERROR: Could not convert vertex -> {:?}", vertex);
        }

        Vertex::ZERO
    }

    const DESCRIPTOR: &'static str = "v";
    const REGEX: &'static str = r"v[ ]*(?P<x>(([-]?\d*\.?\d*)(e-\d*)?)) (?P<y>(([-]?\d*\.?\d*)(e-\d*)?)) (?P<z>(([-]?\d*\.?\d*)(e-\d*)?))";
}

pub struct TextureCoordinate {
    pub u: f32,
    pub v: f32,
    pub w: f32
}

impl TextureCoordinate {
    const ZERO: TextureCoordinate = TextureCoordinate {
        u: 0.0,
        v: 0.0,
        w: 0.0,
    };
}

impl WaveformType for TextureCoordinate {
    fn new(texcoord: String) -> TextureCoordinate {
        lazy_static! {
            static ref TYPE_REGEX: Regex = Regex::new(TextureCoordinate::REGEX).unwrap();
        }

        if let Some(captured_texcoords) = TYPE_REGEX.captures(&texcoord) {
            return TextureCoordinate {
                u: captured_texcoords["u"].parse().unwrap(),
                v: captured_texcoords["v"].parse().unwrap(),
                w: captured_texcoords["w"].parse().unwrap(),
            }
        } else {
            println!("ERROR: Could not convert texcoord -> {:?}", texcoord);
        }

        TextureCoordinate::ZERO
    }

    const DESCRIPTOR: &'static str = "vt";
    const REGEX: &'static str = r"vt[ ]*(?P<u>(([-]?\d*\.?\d*)(e-\d*)?))[ ]*(?P<v>(([-]?\d*\.?\d*)(e-\d*)?))?[ ]*(?P<w>(([-]?\d*\.?\d*)(e-\d*)?))?";
}

pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Normal {
    const ZERO: Normal = Normal {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

impl WaveformType for Normal {
    fn new(normal: String) -> Normal {
        lazy_static! {
            static ref TYPE_REGEX: Regex = Regex::new(Normal::REGEX).unwrap();
        }

        if let Some(captured_normal) = TYPE_REGEX.captures(&normal) {
            return Normal {
                x: captured_normal["x"].parse().unwrap(),
                y: captured_normal["y"].parse().unwrap(),
                z: captured_normal["z"].parse().unwrap(),
            }
        } else {
            println!("ERROR: Could not convert normal -> {:?}", normal);
        }

        Normal::ZERO
    }

    const DESCRIPTOR: &'static str = "vn";
    const REGEX: &'static str = r"vn[ ]*(?P<x>(([-]?\d*\.?\d*)(e-\d*)?)) (?P<y>(([-]?\d*\.?\d*)(e-\d*)?)) (?P<z>(([-]?\d*\.?\d*)(e-\d*)?))";
}

pub struct Face {
    pub point: [u32; 3],
    pub tex: [u32; 3],
    pub norm: [u32; 3],
}

impl WaveformType for Face {
    fn new(face: String) -> Face {
        lazy_static! {
            static ref TYPE_REGEX: Regex = Regex::new(Face::REGEX).unwrap();
        }

        if let Some(captured_face) = TYPE_REGEX.captures(&face) {
            return Face {
                point: [
                    captured_face["px"].parse().unwrap(),
                    captured_face["py"].parse().unwrap(),
                    captured_face["pz"].parse().unwrap(),
                ],
                tex: [
                    captured_face["tx"].parse().unwrap(),
                    captured_face["ty"].parse().unwrap(),
                    captured_face["tz"].parse().unwrap(),
                ],
                norm: [
                    captured_face["nx"].parse().unwrap(),
                    captured_face["ny"].parse().unwrap(),
                    captured_face["nz"].parse().unwrap(),
                ]
            };
        } else {
            println!("ERROR: Could not convert face -> {:?}", face);
        }

        Face {
            point: [0, 0, 0],
            tex: [0, 0, 0],
            norm: [0, 0, 0],
        }
    }

    const DESCRIPTOR: &'static str = "f";
    const REGEX: &'static str = r"f[ ]*(?P<px>\d*)/(?P<tx>\d*)/(?P<nx>\d*) (?P<py>\d*)/(?P<ty>\d*)/(?P<ny>\d*) (?P<pz>\d*)/(?P<tz>\d*)/(?P<nz>\d*)";
}

pub struct Model {
    vertices: Vec<Vertex>,
    texcoords: Vec<TextureCoordinate>,
    normals: Vec<Normal>,
    faces: Vec<Face>,
}

impl Model {
    pub fn new(file: &File) -> Self {
        let mut model = Model {
            vertices: Vec::new(),
            texcoords: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
        };

        let bufreader = BufReader::new(file);

        for line in bufreader.lines() {
            let resolved_line = line.unwrap();
            match resolved_line.split(" ").next().unwrap() {
                Vertex::DESCRIPTOR => {
                    model.vertices.push(Vertex::new(resolved_line));
                }
                TextureCoordinate::DESCRIPTOR => {
                    model.texcoords.push(TextureCoordinate::new(resolved_line));
                }
                Normal::DESCRIPTOR => {
                    model.normals.push(Normal::new(resolved_line));
                }
                Face::DESCRIPTOR => {
                    model.faces.push(Face::new(resolved_line));
                }
                _ => {}
            }
        }

        model
    }

    pub fn iter_faces(&self) -> std::slice::Iter<'_, Face> {
        self.faces.iter()
    }

    pub fn get_normal(&self, i: usize) -> Option<&Normal> {
        self.normals.get(i)
    }

    pub fn get_vertex(&self, i: usize) -> Option<&Vertex> {
        self.vertices.get(i)
    }

    pub fn get_texcoord(&self, i: usize) -> Option<&TextureCoordinate> {
        self.texcoords.get(i)
    }
}
