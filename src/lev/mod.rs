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

pub fn load_lev () {
    //
}

pub fn save_lev (filename: String) {
    //
}
