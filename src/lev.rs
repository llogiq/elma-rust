use std::io::{ Read, Write };
use std::fs::File;
use byteorder::{ ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian };
use super::Position;

// Magic arbitrary number; signifies end-of-data. Followed by Top10 list(s).
const EOD: u32 = 0x0067103A;
// Magic arbitrary number; signifies end-of-file.
const EOF: u32 = 0x00845D52;

/// Type of object.
enum ObjectType {
    Apple,
    Exit,
    Killer,
    Player
}

/// Object struct. Every level requires one ObjectType::Player Object and at least one ObjectType::Exit Object.
pub struct Object {
    /// Position. See Position struct.
    position: Position<f64>,
    /// Type of Object, see ObjectType.
    object_type: ObjectType,
    /// Applies to ObjectType::Apple only.
    ///
    /// 0 = normal
    /// 1 = gravity up
    /// 2 = gravity down
    /// 3 = gravity left
    /// 4 = gravity right
    gravity: u32,
    /// Applies to ObjectType::Apple only. Valid values are 1 to 9.
    animation: u32
}

/// Polygon struct.
pub struct Polygon {
    /// Grass polygon.
    grass: bool,
    /// Vertices in Polygon.
    vertex_count: u32,
    /// Vector with all vertices, see Position struct.
    vertices: Vec<Position<f64>>
}

/// Picture struct.
pub struct Picture {
    /// Picture name.
    name: [u8; 10],
    /// Texture name.
    texture: [u8; 10],
    /// Mask name.
    mask: [u8; 10],
    /// Position. See Position struct.
    position: Position<f64>,
    /// Z-distance
    distance: u32,
    /// Clipping.
    ///
    /// 0 = unclipped
    /// 1 = ground
    /// 2 = sky
    clip: u32
}

/// Level struct that contains all level information.
pub struct Level {
    /// Raw binary data of a loaded or finalized constructed level.
    pub raw: Vec<u8>,
    /// Random number that links level file to replay files.
    pub link: u32,
    /// Contains four integrity checks (See create_integrity()).
    pub integrity: [f64; 4],
    /// Level name.
    pub name: [u8; 51],
    /// LGR file name.
    pub lgr: [u8; 16],
    /// Ground texture name.
    pub ground: [u8; 10],
    /// Sky texture name.
    pub sky: [u8; 10],
    /// Vector with all polygons (See Polygon).
    pub polygons: Vec<Polygon>,
    /// Vector with all objects (See Object).
    pub objects: Vec<Object>,
    /// Vector with all pictures (See Picture).
    pub pictures: Vec<Picture>
}

impl Level {
    /// Returns a new Level struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let level = elma::lev::Level::new();
    /// ```
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

    /// Parses raw binary level data from self.raw and populates Level fields with information.
    fn parse_level (&self) {

    }

    /// Loads a level file and returns a Level struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let level = elma::lev::Level::load_level("tests/test.lev");
    /// ```
    pub fn load_level (filename: &str) -> Level {
        let mut level = Level::new();
        let mut file = File::open(filename).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        level.parse_level();
        level
    }

    /// Converts all struct fields into raw binary form and returns it.
    pub fn get_raw (self) -> Vec<u8> {
        // TODO: convert all fields.
        self.parse_level();
        self.raw
    }

    /// Saves level as a file
    pub fn save_lev (&self, filename: &str) {
        let mut file = File::create(&filename).unwrap();
        file.write_all(b"Hello, world!").unwrap();
    }
}
