use rouille::{Request, Response};
use rouille::input;

use time;

use crate::lang::{Lang, LangSelect};

/// These are the options that can be set for each request and changed through the settings page
pub struct Settings {
    lang: Lang,
}
impl Settings {
    /// Use the cookies from the request to fill out as much as it can, then default the rest
    pub fn new(req: &Request) -> Settings {
        let mut lang = None;
        for (name, value) in input::cookies(req) {
            match name {
                "lang" => lang = LangSelect::from_short_str(value),
                // Do nothing with empty matches
                _ => (),
            }
        }

        Settings {
            // Convert the lang selection that we got from the cookie
            lang: Lang::new(lang.unwrap_or(LangSelect::EN)),
        }
    }
    pub fn lang(&self) -> Lang { self.lang }

    /// Sets the cookies for the settings into the response
    pub fn set_cookies(&self, res: Response) -> Response {
        // Create an expiry date that we can use with every cookie
        let expires = {
            let mut now = time::now_utc();
            // Add 100 years onto the cookie so that it won't expire during use
            now.tm_year += 100;
            now.rfc822().to_string()
        };
        res.with_additional_header("Set-Cookie",
            format!("lang={}; Expires={}; Path=/;", self.lang.lang().short_str(), &expires))
    }
}
