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
    /// Raw binary data of a loaded or finalized constructed level
    pub raw: Vec<u8>,
    /// Random number that links level file to replay files
    pub link: u32,
    /// Contains four integrity checks (See create_integrity())
    pub integrity: [f64; 4],
    /// Level name
    pub name: [u8; 51],
    /// LGR file name
    pub lgr: [u8; 16],
    /// Ground texture name
    pub ground: [u8; 10],
    /// Sky texture name
    pub sky: [u8; 10],
    /// Vector with all polygons (See Polygon)
    pub polygons: Vec<Polygon>,
    /// Vector with all objects (See Object)
    pub objects: Vec<Object>,
    /// Vector with all pictures (See Picture)
    pub pictures: Vec<Picture>
}

impl Level {
    /// Build a new Level
    pub fn new () -> Level {
        Level {
            raw: Vec::new(),
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

    /// Parses raw binary level data from self.raw and populates Level fields with information
    pub fn parse_level (&self) {

    }

    /// Loads level file data into self.raw field then calls self.parse_level()
    pub fn load_level (&mut self, filename: &str) {
        let mut file = File::open(filename).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        self.raw = buffer;
        self.parse_level();
    }

    /// Saves self.raw data to a file
    pub fn save_lev (&self, filename: &str) {
        let mut file = File::create(&filename).unwrap();
        file.write_all(b"Hello, world!").unwrap();
    }
}
