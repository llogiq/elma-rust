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
    name: [u8; 10],
    texture: [u8; 10],
    mask: [u8; 10],
    position: Position,
    distance: u32,
    clip: u32
}

/// Level struct that contains all level information
pub struct Level {
    pub link: u32,
    pub integrity: [f64; 4],
    pub name: [u8; 51],
    pub lgr: [u8; 16],
    pub ground: [u8; 10],
    pub sky: [u8; 10],
    pub polygons: Vec<Polygon>,
    pub objects: Vec<Object>,
    pub pictures: Vec<Picture>
}

impl Level {
    pub fn new () -> Level {
        Level {
            link: 0,
            integrity: [0.0f64; 4],
            name: [0; 51],
            lgr: [0; 16],
            ground: [0; 10],
            sky: [0; 10],
            polygons: Vec::new(),
            objects: Vec::new(),
            pictures: Vec::new()
        }
    }

    fn parse_level (&self, buffer: Vec<u8>) {

    }

    pub fn load_level (&self, filename: &str) {
        let mut file = File::open(filename).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer);
        self.parse_level(buffer);
    }

    pub fn save_lev (&self, filename: &str) {
        let mut file = File::create(&filename).unwrap();
        file.write_all(b"Hello, world!").unwrap();
    }
}
