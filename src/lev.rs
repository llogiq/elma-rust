use std::io::{ Cursor, Read, Write };
use std::fs::File;
use byteorder::{ ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian };
use super::Position;
use super::read_n;
use super::rand::Rng;

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
    /// Elma or Across level.
    pub version: String,
    /// Raw binary data of a loaded or finalized constructed level.
    pub raw: Vec<u8>,
    /// Random number that links level file to replay files.
    pub link: u32,
    /// Contains four integrity checks (See create_integrity()).
    pub integrity: [f64; 4],
    /// Level name.
    pub name: String,
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
            version: String::from("Elma"),
            raw: Vec::new(),
            link: 0,
            integrity: [0.0f64; 4],
            name: String::new(),
            lgr: [0; 16],
            ground: [0; 10],
            sky: [0; 10],
            polygons: Vec::new(),
            objects: Vec::new(),
            pictures: Vec::new()
        }
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
        level.raw = buffer;
        level.parse_level();
        level
    }

    /// Parses the raw binary data into Level struct fields.
    fn parse_level (&mut self) {
        let mut buffer = Cursor::new(&self.raw);
        let data :Vec<u8>;
        // Elma = POT14, Across = POT06.
        // TODO: make Across compatible in 2025.
        let version = read_n(&mut buffer, 5);
        self.version = match version.as_slice() {
            [80, 79, 84, 49, 52] => "Elma".to_string(),
            [80, 79, 84, 48, 54] => "Across".to_string(),
            _ => "".to_string()
        };

        // Link.
        data = read_n(&mut buffer, 2);
        let link = buffer.read_u32::<LittleEndian>().unwrap();
        self.link = link;
    }

    /// Combines the Level struct fields to generate the raw binary data.
    fn convert_to_raw (&self) {
        // TODO: convert duh.
    }

    /// Converts all struct fields into raw binary form and returns it.
    pub fn get_raw (self) -> Vec<u8> {
        self.convert_to_raw();
        self.raw
    }

    /// Saves level as a file.
    pub fn save_lev (&self, filename: &str) {
        self.convert_to_raw();
        let mut file = File::create(&filename).unwrap();
        // TODO: write stuff.
    }
}
