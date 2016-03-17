use std::io::{ Cursor, Read, Write };
use std::fs::File;
use std::ffi::CString;
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
    raw: Vec<u8>,
    /// Random number that links level file to replay files.
    pub link: u32,
    /// Contains four integrity checks (See create_integrity()).
    pub integrity: [f64; 4],
    /// Level name.
    pub name: CString,
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
            version: "Elma".to_string(),
            raw: vec![],
            link: 0,
            integrity: [0.0f64; 4],
            name: CString::new("").unwrap(),
            lgr: [0; 16],
            ground: [0; 10],
            sky: [0; 10],
            polygons: vec![],
            objects: vec![],
            pictures: vec![]
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
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();
        level.raw = buffer;
        level.parse_level();
        level
    }

    /// Parses the raw binary data into Level struct fields.
    fn parse_level (&mut self) {
        let mut buffer = Cursor::new(&self.raw);
        let mut _data :Vec<u8>;

        // Elma = POT14, Across = POT06.
        // TODO: make Across compatible in 2025.
        let version = read_n(&mut buffer, 5);
        self.version = match version.as_slice() {
            [80, 79, 84, 49, 52] => "Elma".to_string(),
            [80, 79, 84, 48, 54] => "Across".to_string(),
            _ => panic!("Not a valid level file.")
        };

        // Link.
        _data = read_n(&mut buffer, 2); // Never used
        self.link = buffer.read_u32::<LittleEndian>().unwrap();

        // Integrity checksums.
        for i in 0..4 {
            self.integrity[i] = buffer.read_f64::<LittleEndian>().unwrap();
        }

        // Level name.
        let mut name :Vec<u8> = vec![];
        let mut finished = false;
        // 51 bytes, read one by one because a null byte can be somewhere in the middle and
        // CString is not a fan of this apparently? Stop pushing onto the 'name' Vec when we
        // hit it, but keep reading the remaining x bytes so we end up at the right place. Eh.
        for n in 0..52 {
            let byte = read_n(&mut buffer, 1);
            if !finished && byte[0] != 0u8 {
                name.push(byte[0]);
            } else if byte[0] == 0u8 {
                finished = true;
            }
        }
        self.name = CString::new(name).unwrap();
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
