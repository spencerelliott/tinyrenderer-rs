extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

trait WaveformType {
    fn new(objString: String) -> Self;

    fn regex() -> Regex {
        Regex::new(Self::REGEX).unwrap()
    }

    const DESCRIPTOR: &'static str;
    const REGEX: &'static str;
}

struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

impl WaveformType for Vertex {
    fn new(vertex: String) -> Vertex {
        let vertex_regex = Self::regex();

        let captured_vertex = vertex_regex.captures(&vertex).unwrap();

        Vertex {
            x: captured_vertex["x"].parse().unwrap(),
            y: captured_vertex["y"].parse().unwrap(),
            z: captured_vertex["z"].parse().unwrap(),
        }
    }

    const DESCRIPTOR: &'static str = "v";
    const REGEX: &'static str = r"v (?P<x>[-]?\d*\.?\d*) (?P<y>[-]?\d*\.?\d*) (?P<z>[-]?\d*\.?\d*)";
}

struct Triangle {
    point: [u32; 3],
    tex: [u32; 3],
    norm: [u32; 3],
}

pub struct Model {
    vertices: Vec<Vertex>,
    faces: Vec<Triangle>,
}

impl Model {
    pub fn new(file: &File) -> Self {
        let vertex_regex =
            Regex::new(r"v (?P<x>\d*.?\d*) (?P<y>\d*.?\d*) (?P<z>\d*.?\d*)").unwrap();
        let face_regex = Regex::new(r"f (?P<px>\d*)/(?P<tx>\d*)/(?P<nx>\d*) (?P<py>\d*)/(?P<ty>\d*)/(?P<ny>\d*) (?P<pz>\d*)/(?P<tz>\d*)/(?P<nz>\d*)").unwrap();

        let mut model = Model {
            vertices: Vec::new(),
            faces: Vec::new(),
        };

        let bufreader = BufReader::new(file);

        for line in bufreader.lines() {
            let resolved_line = line.unwrap();
            match resolved_line.split(" ").next().unwrap() {
                Vertex::DESCRIPTOR => {
                    model.vertices.push(Vertex::new(resolved_line));
                }
                "f" => {}
                _ => {}
            }
        }

        model
    }
}
