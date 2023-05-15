use std::fmt;

/// Enumeration representing the months of the year
///
/// # Example
/// ```
/// use journal_rs::prelude::*;
///
/// let month = Month::DECEMBER;
/// assert!(format!("{}",  month) == "Dec");
/// ```
pub enum Month {
    JANUARY,
    FEBRUARY,
    MARCH,
    APRIL,
    MAY,
    JUNE,
    JULY,
    AUGUST,
    SEPTEMBER,
    OCTOBER,
    NOVEMBER,
    DECEMBER
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let month = match self {
            Self::JANUARY => "Jan",
            Self::FEBRUARY => "Feb",
            Self::MARCH => "Mar",
            Self::APRIL => "Apr",
            Self::MAY => "May",
            Self::JUNE => "Jun",
            Self::JULY => "Jul",
            Self::AUGUST => "Aug",
            Self::SEPTEMBER => "Sep",
            Self::OCTOBER => "Oct",
            Self::NOVEMBER => "Nov",
            Self::DECEMBER => "Dec"
        };
        write!(f, "{}", month)
    }
}

/// Enumeration representing the Days of the week
///
/// # Example
/// ```
/// use journal_rs::prelude::*;
///
/// let day = Weekday::FRIDAY;
/// assert!(format!("{}", day) == "Fri");
/// ```
pub enum Weekday {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let day =  match self {
            Self::MONDAY => "Mon",
            Self::TUESDAY => "Tue",
            Self::WEDNESDAY => "Wed",
            Self::THURSDAY => "Thu",
            Self::FRIDAY => "Fri",
            Self::SATURDAY => "Sat",
            Self::SUNDAY => "Sun",
        };
        write!(f, "{}", day)
    }
}

pub struct Date {
    pub month: Month,
    pub day: (Weekday, u16),
    pub year: u32
}

impl Date {

}

pub struct Page<'a> {
    key: u64,
    title: &'a str,
    body: String,
    date: Date,
    tags: Vec<&'a str>
}

impl<'a> Page<'a> {

}

pub struct PageBuilder<'a> {
    key: u64,
    title: &'a str,
    body: String,
    date: Date,
    tags: Vec<&'a str>
}

impl<'a> PageBuilder<'a> {

}
