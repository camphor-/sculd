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

struct Event {
    start : DateTime<Local>,
    end : DateTime<Local>,
    title : String,
    url : String,
}

fn parse_event(v: &serde_json::Value) -> Event {
    let s = v.get("start").unwrap().as_str().unwrap()
             .parse::<DateTime<Local>>().unwrap();
    let e = v.get("end").unwrap().as_str().unwrap()
             .parse::<DateTime<Local>>().unwrap();
    let t = v.get("title").unwrap().as_str().unwrap().to_string();
    let u = v.get("url").unwrap().as_str().or(Some("None")).unwrap().to_string();

    Event {
        start: s,
        end: e,
        title: t,
        url: u,
    }
}

fn format_event(e: &Event) {
    println!("{} {} - {} {} {}",
             e.start.format("%F"),
             e.start.format("%R"),
             e.end.format("%R"),
             e.title,
             e.url);
}

fn get_ev(name: &str) -> String {
    let err = "Error: Unable to get ".to_string() + name;
    env::var(name).expect(&err)
}

fn get_sched() -> Vec<Event> {
    let url = get_ev("CAMPH_SCULD_URL");
    let user = get_ev("CAMPH_SCULD_USER");
    let pass =  get_ev("CAMPH_SCULD_PASS");

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
    let vs : serde_json::Value = serde_json::from_str(&res_body).unwrap();
    vs.as_array().unwrap().iter().map(parse_event).collect()
}

fn main() {
    let es = get_sched();

    let today = Local::today();
    let next = today + chrono::Duration::days(7);

    let mut res : Vec<&Event> = es.iter().filter(|e| e.start.date() >= today && e.end.date() < next).collect();
    &res.sort_by_key(|e| e.start);
    for e in res {
        format_event(&e);
    }
}
