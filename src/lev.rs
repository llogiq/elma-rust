use std::io::{ Cursor, Read, Write };
use std::fs::File;
use std::ffi::CString;
use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };
use super::{ cstring_read, Position, read_n };
use super::rand::Rng;

// Magic arbitrary number; signifies end-of-data. Followed by Top10 list(s).
const EOD: u32 = 0x0067103A;
// Magic arbitrary number; signifies end-of-file.
const EOF: u32 = 0x00845D52;

/// Type of object.
pub enum ObjectType {
    Apple,
    Exit,
    Killer,
    Player
}

/// Object struct. Every level requires one ObjectType::Player Object and at least one ObjectType::Exit Object.
pub struct Object {
    /// Position. See Position struct.
    pub position: Position<f64>,
    /// Type of Object, see ObjectType.
    pub object_type: ObjectType,
    /// Applies to ObjectType::Apple only.
    ///
    /// 0 = normal
    /// 1 = gravity up
    /// 2 = gravity down
    /// 3 = gravity left
    /// 4 = gravity right
    // TODO: enum with gravity
    pub gravity: u32,
    /// Applies to ObjectType::Apple only. Valid values are 1 to 9.
    pub animation: u32
}

/// Polygon struct.
#[derive(Debug, PartialEq)]
pub struct Polygon {
    /// Grass polygon.
    pub grass: bool,
    /// Vector with all vertices, see Position struct.
    pub vertices: Vec<Position<f64>>
}

impl Polygon {
    pub fn new () -> Polygon {
        Polygon {
            grass: false,
            vertices: vec![]
        }
    }
}

/// Picture struct.
pub struct Picture {
    /// Picture name.
    pub name: CString,
    /// Texture name.
    pub texture: CString,
    /// Mask name.
    pub mask: CString,
    /// Position. See Position struct.
    pub position: Position<f64>,
    /// Z-distance
    pub distance: u32,
    /// Clipping.
    ///
    /// 0 = unclipped
    /// 1 = ground
    /// 2 = sky
    // TODO: make enum
    pub clip: u32
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
    pub lgr: CString,
    /// Ground texture name.
    pub ground: CString,
    /// Sky texture name.
    pub sky: CString,
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
            lgr: CString::new("default").unwrap(),
            ground: CString::new("ground").unwrap(),
            sky: CString::new("sky").unwrap(),
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
        self.name = cstring_read(read_n(&mut buffer, 51));
        // LGR name.
        self.lgr = cstring_read(read_n(&mut buffer, 16));
        // Ground texture name.
        self.ground = cstring_read(read_n(&mut buffer, 10));
        // Sky texture name.
        self.sky = cstring_read(read_n(&mut buffer, 10));

        // Polygons.
        let poly_count = (buffer.read_f64::<LittleEndian>().unwrap() - 0.4643643).round() as u16;
        for _ in 0..poly_count {
            let grass = buffer.read_u32::<LittleEndian>().unwrap() > 0;
            let vertex_count = buffer.read_u32::<LittleEndian>().unwrap();
            let mut vertices: Vec<Position<f64>> = vec![];
            for _ in 0..vertex_count {
                let x = buffer.read_f64::<LittleEndian>().unwrap();
                let y = buffer.read_f64::<LittleEndian>().unwrap();
                vertices.push(Position {
                    x: x,
                    y: y
                });
            }
            self.polygons.push(Polygon {
                grass: grass,
                vertices: vertices
            });
        }

        // Objects.
        let object_count = (buffer.read_f64::<LittleEndian>().unwrap() - 0.4643643).round() as u16;
        for _ in 0..object_count {
            let x = buffer.read_f64::<LittleEndian>().unwrap();
            let y = buffer.read_f64::<LittleEndian>().unwrap();
            let position = Position { x: x, y: y };
            let object_type = match buffer.read_u32::<LittleEndian>().unwrap() {
                1 => ObjectType::Exit,
                2 => ObjectType::Apple,
                3 => ObjectType::Killer,
                4 => ObjectType::Player,
                _ => panic!("Not a valid object type")
            };
            let gravity = buffer.read_u32::<LittleEndian>().unwrap();
            let animation = buffer.read_u32::<LittleEndian>().unwrap();

            self.objects.push(Object {
                position: position,
                object_type: object_type,
                gravity: gravity,
                animation: animation
            });
        }

        // Pictures.
        let picture_count = (buffer.read_f64::<LittleEndian>().unwrap() - 0.2345672).round() as u16;
        for _ in 0..picture_count {
            let name = cstring_read(read_n(&mut buffer, 10));
            let texture = cstring_read(read_n(&mut buffer, 10));
            let mask = cstring_read(read_n(&mut buffer, 10));
            let x = buffer.read_f64::<LittleEndian>().unwrap();
            let y = buffer.read_f64::<LittleEndian>().unwrap();
            let distance = buffer.read_u32::<LittleEndian>().unwrap();
            let clip = buffer.read_u32::<LittleEndian>().unwrap();

            self.pictures.push(Picture {
                name: name,
                texture: texture,
                mask: mask,
                position: Position { x: x, y: y },
                distance: distance,
                clip: clip
            });
        }

        // EOD marker expected at this point.
        let expected = buffer.read_u32::<LittleEndian>().unwrap();
        if expected != EOD { panic!("EOD marker mismatch: x0{:x} != x0{:x}", expected, EOD); }
    }

    /// Combines the Level struct fields to generate the raw binary data.
    fn convert_to_raw (&self) {
        // TODO: convert 8)
    }

    /// Converts all struct fields into raw binary form and returns it.
    pub fn get_raw (self) -> Vec<u8> {
        self.convert_to_raw();
        self.raw
    }

    /// Saves level as a file.
    pub fn save_lev (self, filename: &str) {
        self.convert_to_raw();
        let mut file = File::create(&filename).unwrap();
        // TODO: write stuff.
    }
}
