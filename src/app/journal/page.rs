use std::mem::take;
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
    /// Formats weekday into text
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
    /// Formats date into text 
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}, {} {}, {}", self.day.0, self.month, self.day.1, self.year) 
    }
}

/// Creates a defaulted to Monday, January 1, 1900
///
/// # Example
/// ```
/// use journal_rs::prelude::*;
///
/// let date = Date::default();
///
/// assert!(format!("{}", date) == "Mon, Jan 1, 1900");
/// ```
impl Default for Date {
    /// Returns a date, set to Monday, January 1, 1900
    fn default() -> Self {
        Self {
            month: Month::JANUARY,
            day: (Weekday::MONDAY, 1),
            year: 1900
        }
    }
}

impl Date {
    /// 
    pub fn now() -> Date {
        todo!() 
    }
}

/// Structure representing a page in a journal
///
/// # Example 
/// ```
/// use journal_rs::prelude::*;
///
/// let page = Page::builder()
///     .key(1)
///     .title("Test")
///     .body("Hello world!")
///     .date(Date::default())
///     .tag("exciting")
///     .build();
///
/// assert!(page.key() == 1);
/// assert!(page.title() == "Test");
/// assert!(page.body() == "Hello world!");
/// assert!(format!("{}", page.date()) == "Mon, Jan 1, 1900");
/// assert!(page.tag(0) == "exciting");
/// ```
pub struct Page {
    key: usize,
    title: String,
    body: String,
    date: Date,
    tags: Vec<String>
}

impl<'a> Page {
    /// Returns a pointer to a new PageBuilder instance
    pub fn builder() -> Box<PageBuilder> {
        Box::new(PageBuilder { 
            key: Default::default(), 
            title: String::new(), 
            body: String::new(), 
            date: Default::default(), 
            tags: Vec::new() 
        })
    }

    /// Returns the key of the page
    pub fn key(&self) -> usize {
        self.key
    }
    
    /// Returns the title of the page
    pub fn title(&'a self) -> &'a str {
        &self.title[..]
    }

    /// Returns the body of the page
    pub fn body(&'a self) -> &'a str {
        &self.body[..]
    }

    /// Returns the date the page was created
    pub fn date(&'a self) -> &'a Date {
        &self.date
    }

    /// Returns the tag at the given index
    pub fn tag(&'a self, key: usize) -> &'a str {
        &self.tags[key]
    }

    /// Returns a immutable reference to the tags vector
    pub fn tags(&'a self) -> &'a Vec<String> {
        &self.tags
    }
}

/// Builder structure used for creating a new page
///
/// # Example
/// ```
/// use journal_rs::prelude::*;
///
/// let page = Page::builder()
///     .key(1)
///     .title("Test")
///     .body("Hello world!")
///     .date(Date::default())
///     .tag("exciting")
///     .build();
///
/// assert!(page.key() == 1);
/// assert!(page.title() == "Test");
/// assert!(page.body() == "Hello world!");
/// assert!(format!("{}", page.date()) == "Mon, Jan 1, 1900");
/// assert!(page.tag(0) == "exciting");
/// ```
pub struct PageBuilder {
    key: usize,
    title: String,
    body: String,
    date: Date,
    tags: Vec<String>
}

impl PageBuilder {
    // Sets the key of the page being built
    pub fn key(&mut self, key: usize) -> &mut Self {
        self.key = key;
        self
    }

    /// Sets the title of the page being built
    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();
        self
    }

    /// Sets the body of the page being built
    pub fn body(&mut self, body: &str) -> &mut Self {
        self.body = body.to_string();
        self
    }

    /// Sets the date of the page being built
    pub fn date(&mut self, date: Date) -> &mut Self {
        self.date = date;
        self
    }
    
    /// Adds a tag to the page being built
    pub fn tag(&mut self, tag: &str) -> &mut Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Takes the page builder and moves its attributes to a new Page instance and returns it
    pub fn build(&mut self) -> Page {
        Page {
            key: take(&mut self.key),
            title: take(&mut self.title),
            body: take(&mut self.body),
            date: take(&mut self.date),
            tags: take(&mut self.tags)
        }
    }
}
