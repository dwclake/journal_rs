pub trait ToSTR<'a> {
    fn to_str(self) -> &'a str;
}

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

impl<'a> ToSTR<'a> for Month {
    fn to_str(self) -> &'a str {
        return match self {
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
        }
    }
}

pub enum Weekday {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY
}

impl<'a> ToSTR<'a> for Weekday {
    fn to_str(self) -> &'a str {
        return match self {
            Self::MONDAY => "Mon",
            Self::TUESDAY => "Tue",
            Self::WEDNESDAY => "Wed",
            Self::THURSDAY => "Thu",
            Self::FRIDAY => "Fri",
            Self::SATURDAY => "Sat",
            Self::SUNDAY => "Sun",
        }
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
