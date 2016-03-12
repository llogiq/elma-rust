use std::io::Read;
use std::io::Write;
use std::fs::File;

const EOD: u32 = 0x0067103A;
const EOF: u32 = 0x00845D52;

/// Position struct shared between Object, Polygon and Picture
pub struct Position {
    x: f64,
    y: f64
}

/// Type of object
enum ObjectType {
    Apple,
    Exit,
    Killer,
    Player
}

pub struct Object {
    position: Position,
    object_type: ObjectType,
    gravity: u32,
    animation: u32
}

pub struct Polygon {
    grass: bool,
    vertex_count: u32,
    vertices: Vec<Position>
}

pub struct Picture {
    name: String,
    texture: String,
    mask: String,
    position: Position,
    distance: u32,
    clip: u32
}

/// Level struct that contains all level information
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

    pub fn load_level (&self, filename: &str) {
        let mut file = File::open(filename).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
    }

    pub fn save_lev (&self, filename: &str) {
        let mut file = File::create(&filename).unwrap();
        file.write_all(b"Hello, world!").unwrap();
    }
}
