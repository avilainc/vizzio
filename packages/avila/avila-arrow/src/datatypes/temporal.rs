//! Temporal data types: Date, Time, Duration, Interval

/// Date types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateType {
    Date32,
    Date64,
}

/// Time types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeType {
    Time32(TimeUnit),
    Time64(TimeUnit),
}

/// Time units
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
}

/// Duration type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurationType {
    pub unit: TimeUnit,
}

/// Interval type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntervalType {
    YearMonth,
    DayTime,
    MonthDayNano,
}
