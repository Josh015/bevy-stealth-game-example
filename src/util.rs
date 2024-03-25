use std::f32::consts::*;

/// Controls how many times a thing can repeat.
pub enum Repeat {
    /// Repeats an unlimited number of times.
    #[allow(dead_code)]
    Forever,

    /// Repeats for a specified number of times.
    Times(u32),
}

impl Repeat {
    /// Says whether the item is finished repeating.
    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Times(0))
    }

    /// Reduces the number of times the item will repeat by 1. Does nothing if
    /// the item has already reached its limit or is set to repeat forever.
    pub fn advance(&mut self) {
        match self {
            Repeat::Forever | Repeat::Times(0) => {},
            Repeat::Times(ref mut repeat) => {
                *repeat -= 1;
            },
        };
    }
}

/// Takes an angle outside `[-PI, PI]` and remaps it to an equivalent valid
/// angle within that range.
pub fn wrap_angle(angle_radians: f32) -> f32 {
    let mut new_angle = angle_radians;

    while new_angle > PI {
        new_angle -= TAU;
    }

    while new_angle <= -PI {
        new_angle += TAU;
    }

    return new_angle;
}
