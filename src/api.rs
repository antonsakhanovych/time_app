use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Deserialize, Serialize)]
pub struct DateResponse {
    abbreviation: String,
    client_ip: String,
    datetime: String,
    day_of_week: isize,
    day_of_year: isize,
    dst: bool,
    dst_from: String,
    dst_offset: isize,
    dst_until: String,
    raw_offset: isize,
    timezone: String,
    unixtime: i128,
    utc_datetime: String,
    utc_offset: String,
    week_number: isize,
}

impl Display for DateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parsed_date = iso8601::datetime(&self.datetime).unwrap_or_default();
        let time_obj = parsed_date.time;
        let formatted_time = format!("{}:{}:{}", time_obj.hour, time_obj.minute, time_obj.second);
        write!(
            f,
            "{} - {} - {} - {} ",
            self.timezone, self.abbreviation, formatted_time, parsed_date.date
        )
    }
}

pub struct RequestSender {
    api_url: String,
}

impl RequestSender {
    pub fn new(api_url: &str) -> Self {
        RequestSender {
            api_url: api_url.to_string(),
        }
    }
    pub async fn make_request(
        &self,
        zone: &str,
    ) -> Result<DateResponse, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}", self.api_url, zone);
        let resp = reqwest::get(request_url).await?.text().await?;
        let resp: DateResponse = serde_json::from_str(&resp)?;
        Ok(resp)
    }
}
