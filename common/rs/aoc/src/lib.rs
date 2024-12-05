use std::fs;
use std::path::PathBuf;

use chrono::{FixedOffset, NaiveDate, Utc, TimeZone};

use dirs::config_dir;

use reqwest::{
    blocking::Client as HttpClient,
    header::{self, HeaderValue, HeaderMap},
    redirect::Policy,
};

fn is_day_unlocked(year: i32, day: u32) -> bool {
    let timezone = FixedOffset::east_opt(-5 * 3600).unwrap();
    let now = timezone.from_utc_datetime(&Utc::now().naive_utc());

    let local_datetime = NaiveDate::from_ymd_opt(year, 12, day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let unlock_datetime = timezone
        .from_local_datetime(&local_datetime)
        .single()
        .unwrap();
    
    now.signed_duration_since(&unlock_datetime).num_milliseconds() >= 0
}

pub fn get_input(year: i32, day: u32, input: &str) {
    let input = PathBuf::from(input);

    let input_data = fs::read_to_string(&input);
    
    let input_is_missing = input_data
        .as_ref()
        .map(|content| content.is_empty())
        .unwrap_or(true);
    if !input_is_missing {
        return;
    }

    if !is_day_unlocked(year, day) {
        let data = "";
        let must_write = input_data.map(|content| &content != data).unwrap_or(true);
        if must_write {
            fs::write(&input, &data).expect("cannot write (default) input file");
        }
        return;
    }
    
    let session_key = fs::read_to_string(config_dir()
                                         .expect("cannot find config dir")
                                         .join("adventofcode.session"))
        .expect("cannot read session key");

    let cookie_header = HeaderValue::from_str(&format!("session={}", session_key.trim()))
        .unwrap();
    let content_type_header = HeaderValue::from_str("text/plain").unwrap();
    let user_agent_header = HeaderValue::from_str(&format!("{} {}", env!("CARGO_PKG_REPOSITORY"), env!("CARGO_PKG_VERSION"))).unwrap();

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let mut headers = HeaderMap::new();
    headers.insert(header::COOKIE, cookie_header);
    headers.insert(header::CONTENT_TYPE, content_type_header);
    headers.insert(header::USER_AGENT, user_agent_header);

    let data = HttpClient::builder()
        .default_headers(headers)
        .redirect(Policy::none())
        .build()
        .unwrap()
        .get(url)
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.text())
        .unwrap();

    let must_write = input_data.map(|content| content != data).unwrap_or(true);
    if must_write {
        fs::write(&input, &data).expect("cannot write (default) input file");
    }
}
