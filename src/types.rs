use regex::{Regex};

use serde_derive::{Deserialize, Serialize};

use crate::lang::{Lang};

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
        let matches = Lang::all().into_iter().find_map(|lang| {
            // Create the regex for this language
            let pattern = Regex::new(Date::date_format(lang, false)).unwrap();
            // Try to get the matches for this language
            pattern.captures(date)
        });
        if let Some(matches) = matches {
            // Since we found a good match, we just need to see if its values are good
            let year = matches[1].parse().unwrap();
            let month = matches[2].parse().unwrap();
            let day = matches[3].parse().unwrap();
            Date::new(year, month, day)
        } else {
            Err(DateError::BadFormat)
        }
    }

    /// Returns the date format for a specific language.
    /// If user_facing, a string that can be shown to users will be given
    pub fn date_format(lang: Lang, user_facing: bool) -> &'static str {
        if user_facing {
            match lang {
                Lang::EN => "YYYY-MM-DD",
                Lang::JP => "YYYY年MM月DD日",
            }
        } else {
            match lang {
                Lang::EN => r"(\d\d\d\d)-(\d\d)-(\d\d)",
                Lang::JP => r"(\d\d\d\d)年(\d\d)月(\d\d)日",
            }
        }
    }

    /// Gets the language representation of this date
    pub fn lang_str(&self, lang: Lang) -> String {
        match lang {
            Lang::EN => format!("{:04}-{:02}-{:02}", self.year, self.month, self.day),
            Lang::JP => format!("{:04}年{:02}月{:02}日", self.year, self.month, self.day),
        }
    }
}
/// The errors that may occur when parsing a date
pub enum DateError {
    BadFormat,
    InvalidDay,
    InvalidMonth,
}
