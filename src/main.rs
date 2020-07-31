#[macro_use]
extern crate clap;
use clap::App;
use reqwest::{blocking, header};

/// The type representing a URl
type Url = &'static str;

/// The URL to request for a shortened weather view
const SHORT_URL: Url = "https://wttr.in/?0TFQ";
// The URL to request for a full weather view
const LONG_URL: Url = "https://wttr.in/";
// The URL for version 2
const PRETTY_URL: Url = "https://v2.wttr.in/";

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();

    let url = if matches.is_present("short") {
        SHORT_URL
    } else if matches.is_present("pretty") {
        PRETTY_URL
    } else {
        LONG_URL
    };

    let response = blocking::Client::new()
        .get(url)
        .header(header::USER_AGENT, "curl")
        .send();
    let response = match response {
        Ok(res) => res,
        Err(err) => panic!(
            "An error occurred when trying to request the weather: {:#?}",
            err
        ),
    };

    let text = match response.text() {
        Ok(text) => text,
        Err(err) => panic!(
            "An error occurred when trying to parse the response: {:#?}",
            err
        ),
    };

    println!("{}", text);
}
