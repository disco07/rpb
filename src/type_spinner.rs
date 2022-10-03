use strum::{EnumIter, Display};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, EnumIter, Display)]
pub enum Spinners {
    Dots,
    Dots2,
    Dots3,
    Dots4,
    Line,
    GrowVertical,
    Bounce,
    Triangle,
    CircleHalves,
    Arrow,
    Clock,
    Earth,
    Moon,
    Weather,
    Point,
}