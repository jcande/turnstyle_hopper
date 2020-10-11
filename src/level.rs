use std::collections::HashSet;

use crate::types::{Coord, VertexProperty};

#[derive(Debug)]
pub struct Level {
    pub pieces: HashSet<VertexProperty>,

    pub start: Coord,
    pub end: Coord,

    pub capacity: usize,

    pub width: usize,
    pub height: usize,
}

impl Level {
    pub fn default() -> Level {
        const DEFAULT_WIDTH: usize = 11;
        const DEFAULT_HEIGHT: usize = 11;

        // Middle square on the far left.
        let DEFAULT_START: Coord = Coord::new(0, 5);
        // Middle square on the far right.
        let DEFAULT_END: Coord = Coord::new(10, 5);

        const DEFAULT_CAPACITY: usize = 1;

        let DEFAULT_PIECES: HashSet<VertexProperty> = HashSet::new();

        Level {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,

            start: DEFAULT_START,
            end: DEFAULT_END,

            capacity: DEFAULT_CAPACITY,

            pieces: DEFAULT_PIECES,
        }
    }

    pub fn with_dimension(mut self, width: usize, height: usize) -> Level {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_pieces(mut self, pieces: Vec<VertexProperty>) -> Level {
        // assert!() all pieces.coord within self.height/self.width
        // assert!() all pieces are unique (pieces.len() same as hashset.len())
        self.pieces = pieces.into_iter().collect();
        self
    }

    pub fn with_capacity(mut self, capacity: usize) -> Level {
        self.capacity = capacity;
        self
    }

    pub fn with_start(mut self, start: Coord) -> Level {
        self.start = start;
        self
    }

    pub fn with_end(mut self, end: Coord) -> Level {
        self.end = end;
        self
    }
}
