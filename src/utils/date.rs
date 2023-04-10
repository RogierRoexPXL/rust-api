use chrono::{DateTime, Utc, ParseError, NaiveDate};

static FORMAT: &str = "%Y-%m-%d";

// example:
// let date_string = "2023-04-10T12:34:56.789Z";
// match map_string_to_date(date_string) {
//     Ok(parsed_date) => println!("Parsed date: {}", parsed_date),
//     Err(e) => eprintln!("Error: {}", e),
pub fn map_string_to_date(date: &str) -> Result<DateTime<Utc>, ParseError> {
    DateTime::parse_from_str(date, FORMAT)
        .map(|dt| dt.with_timezone(&Utc))
}

// Example
// let date = Utc::now();
// let date_string = map_date_to_string(&date);
pub fn map_date_to_string(date: &DateTime<Utc>) -> String {
    date.format(FORMAT).to_string()
}

pub fn is_valid_date_format(date: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(date, FORMAT)
}