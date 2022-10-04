use strum;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, strum::EnumIter, strum::Display)]
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
    Men,
    Weather,
    Point,
}