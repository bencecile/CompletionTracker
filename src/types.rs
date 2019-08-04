use serde_derive::{Deserialize, Serialize};

use crate::lang::{UILang};

/// This is a generic date for year, month, date
#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Date {
    year: u32,
    month: u8,
    day: u8,
}
impl Date {
    /// Tries to create a new date
    pub fn new(year: u32, month: u8, day: u8) -> Result<Date, DateError> {
        // Check the month range
        if month < 1 || month > 12 {
            return Err(DateError::InvalidMonth);
        }

        // Finds the max day for this month and year
        let max_day = {
            // Create a table with the days of the month
            let mut day_table = [
                // We made sure that our month starts at 1
                0,
                31 /*Jan*/, 28 /*Feb*/, 31 /*Mar*/, 30 /*Apr*/, 31 /*May*/, 30 /*Jun*/,
                31 /*Jul*/, 30 /*Aug*/, 30 /*Sep*/, 31 /*Oct*/, 30 /*Nov*/, 31 /*Dec*/,
            ];
            // A leap year is every 4 years and every 400, but not every 100
            if (year % 4 == 0) & (year % 100 != 0 || year % 400 == 0) {
                // We are in a leap year, so adjust February
                day_table[2] += 1;
            }
            // Now we can lookup the days in the month
            day_table[month as usize]
        };
        // Check that the day is valid (a little bit trickier)
        if day < 1 || day > max_day {
            return Err(DateError::InvalidDay);
        }

        Ok(Date {
            year,
            month,
            day,
        })
    }
    /// Tries to parse a date out of a string
    pub fn parse_date(date: &str) -> Result<Date, DateError> {
        // Find the matching format if we can, from the different languages
        let matches = UILang::all().iter().find_map(|lang| {
            let format = Date::date_format(*lang);
            // Check for an exact length match
            if format.chars().count() != date.chars().count() {
                return None;
            }

            // Use the format as a spec for each year, month and day locations in the string
            let mut year = String::new();
            let mut month = String::new();
            let mut day = String::new();
            // Go through the string, finding the matching locations
            for (format_char, date_char) in format.chars().zip(date.chars()) {
                match format_char {
                    'Y' => year.push(date_char),
                    'M' => month.push(date_char),
                    'D' => day.push(date_char),
                    _ => (),
                }
            }

            Some( (year, month, day) )
        });
        if let Some( (year, month, day) ) = matches {
            let year = year.parse().map_err(|_| DateError::InvalidYear)?;
            let month = month.parse().map_err(|_| DateError::InvalidMonth)?;
            let day = day.parse().map_err(|_| DateError::InvalidDay)?;
            Date::new(year, month, day)
        } else {
            Err(DateError::BadFormat)
        }
    }

    /// Returns the date format for a specific language.
    /// If user_facing, a string that can be shown to users will be given
    pub fn date_format(lang: UILang) -> &'static str {
        match lang {
            UILang::EN => "YYYY-MM-DD",
            UILang::JP => "YYYY年MM月DD日",
        }
    }

    /// Gets the language representation of this date
    pub fn lang_str(&self, lang: UILang) -> String {
        match lang {
            UILang::EN => format!("{:04}-{:02}-{:02}", self.year, self.month, self.day),
            UILang::JP => format!("{:04}年{:02}月{:02}日", self.year, self.month, self.day),
        }
    }
}
/// The errors that may occur when parsing a date
pub enum DateError {
    BadFormat,
    InvalidDay,
    InvalidMonth,
    InvalidYear,
}
