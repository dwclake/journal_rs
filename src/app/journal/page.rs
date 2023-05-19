use std::fmt::{Formatter, Display, Result};

/// Enumeration representing the months of the year
///
/// # Example
/// ```
/// use journal_rs::prelude::*;
///
/// let month = Month::DECEMBER;
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


/// Allows Month enumerations to be formatted into text
///
/// # Example
/// ```
/// use journal_rs::prelude::*;
///
/// let month = Month::DECEMBER;
/// assert!(format!("{}",  month) == "Dec");
/// ```
impl Display for Month {
    fn fmt(&self, f: &mut Formatter) -> Result {
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

/// Allows Weekday enumerations to be formatted into text
///
/// # Example
/// ```
/// use journal_rs::prelude::*;
///
/// let day = Weekday::FRIDAY;
/// assert!(format!("{}", day) == "Fri");
/// ```
impl Display for Weekday {
    fn fmt(&self, f: &mut Formatter) -> Result {
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

/// Struct representing a date 
///
/// # Example
/// ```
/// use journal_rs::prelude::*;
///
/// let date = Date {
///    month: Month::DECEMBER,
///    day: (Weekday::FRIDAY, 25),
///    year: 2020
/// };
/// ```
pub struct Date {
    pub month: Month,
    pub day: (Weekday, u16),
    pub year: u32
}

/// Allows Date structures to be formatted into text
///
/// # Example
/// ```
/// use journal_rs::prelude::*;
///
/// let date = Date {
///    month: Month::DECEMBER,
///    day: (Weekday::FRIDAY, 25),
///    year: 2020
/// };
///
/// assert!(format!("{}", date) == "Fri, Dec 25, 2020");
/// ```
impl Display for Date {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}, {} {}, {}", self.day.0, self.month, self.day.1, self.year) 
    }
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
