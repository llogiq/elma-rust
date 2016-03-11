use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;


struct Position {
    x: i64,
    y: i64
}

enum ObjectType {
    Apple,
    Exit,
    Killer,
    Picture,
    Player
}

struct Object {
    object_type: ObjectType,
    position: Position
}

struct Polygon {
    points: Vec<Position>,
    grass: bool
}

struct Level {
    name: String,
    ground: String,
    sky: String,
    polygons: Vec<Polygon>,
    objects: Vec<Object>
}

impl Level {
    pub fn load_level (&self, filename: &str) {
        let mut file = File::open(&filename).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s);
    }

    pub fn save_lev (&self, filename: &str) {
        let mut file = File::create(&filename).unwrap();
        file.write_all(b"Hello, world!");
    }
}
