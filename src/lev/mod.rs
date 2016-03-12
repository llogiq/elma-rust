use std::io;
use std::io::prelude::*;
use std::fs::File;

const EOD: u32 = 0x0067103A;
const EOF: u32 = 0x00845D52;

pub struct Position {
    x: f64,
    y: f64
}

enum ObjectType {
    Apple,
    Exit,
    Killer,
    Picture,
    Player
}

pub struct Object {
    object_type: ObjectType,
    position: Position
}

pub struct Polygon {
    points: Vec<Position>,
    grass: bool
}

pub struct Level {
    pub link: u32,
    pub integrity: Vec<f64>,
    pub name: String,
    pub lgr: String,
    pub ground: String,
    pub sky: String,
    pub polygons: Vec<Polygon>,
    pub objects: Vec<Object>
}

impl Level {
    pub fn new () -> Level {
        Level {
            link: 0,
            integrity: Vec::new(),
            name: String::new(),
            lgr: String::new(),
            ground: String::new(),
            sky: String::new(),
            polygons: Vec::new(),
            objects: Vec::new()
        }
    }

    pub fn load_level (filename: &str) {
        let mut file = File::open(&filename).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s);
    }

    pub fn save_lev (&self, filename: &str) {
        let mut file = File::create(&filename).unwrap();
        file.write_all(b"Hello, world!");
    }
}
