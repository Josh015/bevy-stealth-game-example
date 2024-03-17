/// Controls how many times a thing can repeat.
pub enum Repeat {
    Forever,
    Times(u32),
}

impl Repeat {
    pub fn is_finished(&self) -> bool { matches!(self, Self::Times(0)) }

    pub fn advance(&mut self) {
        match self {
            Repeat::Forever | Repeat::Times(0) => {},
            Repeat::Times(ref mut repeat) => {
                *repeat -= 1;
            },
        };
    }
}
