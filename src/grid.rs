use crate::coordinate::Coordinate;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "cbindgen-on", repr(C))]
pub struct Grid {
    width: usize,
    height: usize,

    obstacles: Vec<Vec<bool>>,
}

impl Grid {
    pub(crate) fn from_int_vecs(width: usize, height: usize, i: Vec<Vec<i64>>) -> Grid {
        Self {
            width,
            height,
            obstacles: i.into_iter()
                .map(|r| r.into_iter()
                    .map(|b| if b == 0 { false } else { true } )
                    .collect()
                )
                .collect(),
        }
    }
}

impl Grid {
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn wall_at(&self, at: Coordinate) -> Option<bool> {
        if at.x < 0 || at.x >= self.width as i64 || at.y < 0 || at.y >= self.height as i64 {
            return None
        } else {
            Some(self.obstacles[at.y as usize][at.x as usize])
        }
    }
}