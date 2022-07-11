#![allow(dead_code, unused_variables)]

use crate::secret::{ACCOUNT_SID, AUTH_TOKEN};
use reqwest;
use serde_json::{self, Value};
use tokio::*;
use twilio_async::{Twilio, TwilioRequest};

mod secret;

const OWM_ENDPOINT: &str = "https://api.openweathermap.org/data/2.5/onecall?";
const LAT: &str = "lat=41.6562204&";
const LON: &str = "lon=-70.4161364&";
const EXCLUDE: &str = "exclude=minutely,daily,current&";
const APPID: &str = "appid=";
const ID: &str = secret::API_KEY;

#[main]
async fn main() {
    if get_weather_data().await {
        println!("Rain!")
    } else {
        println!("No Rain")
    }
    test_email().await;
}

async fn get_weather_data() -> bool {
    let url = String::new();
    let url = url + OWM_ENDPOINT + LAT + LON + EXCLUDE + APPID + ID;
    println!("{}", url);

    let body = reqwest::get(url)
        .await
        .expect("error with get request")
        .text()
        .await
        .expect("error with converting to text");

    let v: Value = serde_json::from_str(&body).expect("failed to parse into Value");

    let mut will_rain = false;
    let hours = 0..=23;
    for hour in hours {
        let num = &v["hourly"][hour]["weather"][0]["id"];
        // println!("{}", &num);
        match num.as_i64() {
            x if x.unwrap() < 600 => will_rain = true,
            _ => will_rain = false,
        }
    }
    will_rain
}

async fn test_email() {
    let to = "+15083606350";
    let from = "+18456608853";
    let body = "Bring an umbrella, rain is forecast in the next 12 hours.";
    let app_id = ACCOUNT_SID;
    let auth_token = AUTH_TOKEN;
    let client = Twilio::new(app_id, auth_token).expect("failed to create twilio client");
    match client.send_msg(from, to, body).run().await {
        Ok(m) => println!("{:?}", m),
        Err(e) => eprintln!("{:?}", e),
    }
}
