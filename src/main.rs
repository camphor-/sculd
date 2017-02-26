extern crate chrono;
extern crate hyper;
extern crate hyper_rustls;
extern crate serde_json;

use std::env;
use std::io::Read;
use chrono::prelude::*;
use chrono::Duration;
use serde_json::Value;
use hyper::client::*;
use hyper::header::*;
use hyper::net::*;

fn main() {
    let url = env::var("CAMPH_SCHED_URL").expect("Error");
    let user = env::var("CAMPH_SCHED_USER").expect("Error");
    let pass = env::var("CAMPH_SCHED_PASS").expect("Error");

    let ssl = hyper_rustls::TlsClient::new();
    let conn = HttpsConnector::new(ssl);
    let client = Client::with_connector(conn);

    let auth = Authorization(Basic {
        username: user,
        password: Some(pass),
    });
    let mut res = client.get(&url).header(auth).send().unwrap();
    let mut res_body = String::new();
    res.read_to_string(&mut res_body);
    let vs: Value = serde_json::from_str(&res_body).unwrap();

    let today = Local::today();
    let next = today + Duration::days(7);

    for v in vs.as_array().unwrap() {
        let s = v.get("start").unwrap().as_str().unwrap();
        let sdt = s.parse::<DateTime<Local>>().unwrap();
        let e = v.get("end").unwrap().as_str().unwrap();
        let edt = e.parse::<DateTime<Local>>().unwrap();
        let t = v.get("title").unwrap().as_str().unwrap();
        let u = v.get("url").unwrap().as_str().or(Some("None")).unwrap();

        if sdt.date() >= today && edt.date() < next {
            println!("{} {} - {} {} {}",
                     sdt.format("%F"),
                     sdt.format("%R"),
                     edt.format("%R"),
                     t,
                     u);
        }
    }
}
