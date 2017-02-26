extern crate chrono;
extern crate hyper;
extern crate hyper_rustls;
extern crate serde_json;

use std::env;
use std::io::Read;
use chrono::prelude::{DateTime, Local};
use hyper::client::Client;
use hyper::header::{Authorization, Basic};
use hyper::net::HttpsConnector;

fn get_ev(name: &str) -> String {
    let err = "Error: Unable to get ".to_string() + name;
    env::var(name).expect(&err)
}

fn get_sched() -> serde_json::Value {
    let url = get_ev("CAMPH_SCHED_URL");
    let user = get_ev("CAMPH_SCHED_USER");
    let pass =  get_ev("CAMPH_SCHED_PASS");

    let ssl = hyper_rustls::TlsClient::new();
    let conn = HttpsConnector::new(ssl);
    let client = Client::with_connector(conn);

    let auth = Authorization(Basic {
        username: user,
        password: Some(pass),
    });
    let mut res = client.get(&url).header(auth).send()
                        .expect("Error: Unable to get schedule");
    let mut res_body = String::new();
    res.read_to_string(&mut res_body).unwrap();
    serde_json::from_str(&res_body).unwrap()
}

fn main() {
    let vs = get_sched();

    let today = Local::today();
    let next = today + chrono::Duration::days(7);

    for v in vs.as_array().unwrap() {
        let s = v.get("start").unwrap().as_str().unwrap()
                 .parse::<DateTime<Local>>().unwrap();
        let e = v.get("end").unwrap().as_str().unwrap()
                 .parse::<DateTime<Local>>().unwrap();
        let t = v.get("title").unwrap().as_str().unwrap();
        let u = v.get("url").unwrap().as_str().or(Some("None")).unwrap();

        if s.date() >= today && e.date() < next {
            println!("{} {} - {} {} {}",
                     s.format("%F"),
                     s.format("%R"),
                     e.format("%R"),
                     t,
                     u);
        }
    }
}
