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

pub enum Weekday {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY
}

pub struct Date {
    pub month: Month,
    pub day: (Weekday, u16),
    pub year: u32
}

pub struct Page<'a> {
    key: u64,
    title: &'a str,
    body: String,
    date: Date,
    tags: Vec<&'a str>
}

pub struct PageBuilder<'a> {
    key: u64,
    title: &'a str,
    body: String,
    date: Date,
    tags: Vec<&'a str>
}

impl Date {

}

impl<'a> Page<'a> {

}

impl<'a> PageBuilder<'a> {

}
