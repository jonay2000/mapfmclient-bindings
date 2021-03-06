use crate::marked::MarkedCoordinate;
use crate::grid::Grid;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "cbindgen-on", repr(C))]
pub struct Problem {
    pub grid: Grid,

    pub starts: Vec<MarkedCoordinate>,
    pub goals: Vec<MarkedCoordinate>,
}
