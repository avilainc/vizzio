//! Physical units system

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Meter,
    Kilogram,
    Second,
    Ampere,
    Kelvin,
    Mole,
    Candela,
}

#[derive(Debug, Clone)]
pub struct Quantity {
    pub value: f64,
    pub unit: Unit,
}

impl Quantity {
    pub fn new(value: f64, unit: Unit) -> Self {
        Self { value, unit }
    }

    pub fn meters(value: f64) -> Self {
        Self::new(value, Unit::Meter)
    }

    pub fn kilograms(value: f64) -> Self {
        Self::new(value, Unit::Kilogram)
    }

    pub fn seconds(value: f64) -> Self {
        Self::new(value, Unit::Second)
    }
}
