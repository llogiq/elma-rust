use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use super::Position;

// Magic arbitrary number to signify end of replay file.
const EOR: u32 = 0x00492F75;

/// One frame of replay.
struct Frame {
    /// Bike position?
    bike: Position<f32>,
    /// Left wheel position.
    left_wheel: Position<i16>,
    /// Right wheel position.
    right_wheel: Position<i16>,
    /// Head position.
    head: Position<i16>,
    /// Bike rotation. Range 0..10000.
    rotation: i16,
    /// Left wheel rotation. Range 0..255.
    left_wheel_rotation: u8,
    /// Right wheel rotation. Range 0..255.
    right_wheel_rotation: u8,
    /// Throttle.
    throttle: bool,
    /// Right direction. True = right, False = left.
    right: bool,
    /// Spring sound effect volume.
    volume: u16
}

struct Event {
    /// Time of event.
    time: f64,
    /// Event type.
    event_type: [u32; 2]
}

/// Rec struct
pub struct Rec {
    /// Number of Frames in replay.
    frame_count: i32,
    /// Whether replay is multi-player or not.
    multi: bool,
    /// Whether replay is flag-tag or not.
    flag_tag: bool,
    /// Random number to link with level file.
    link: u32,
    /// Full level filename.
    level: [u8; 12],
    /// Vector with Frame structs.
    frames: Vec<Frame>,
    /// Number of replay events.
    event_count: i32,
    /// Events.
    events: Vec<Event>
}

impl Rec {
    /// Build a new Rec.
    ///
    /// # Examples
    ///
    /// ```
    /// let rec = elma::rec::Rec::new();
    /// ```
    pub fn new () -> Rec {
        Rec {
            frame_count: 0,
            multi: false,
            flag_tag: false,
            link: 0,
            level: [0; 12],
            frames: Vec::new(),
            event_count: 0,
            events: Vec::new()
        }
    }
}
