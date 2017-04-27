extern crate chrono;
extern crate hyper;
extern crate hyper_rustls;
extern crate serde_json;

use std::env;
use std::fmt;
use std::io::Read;
use std::process;
use chrono::prelude::{DateTime, Local};
use hyper::client::Client;
use hyper::header::{Authorization, Basic};
use hyper::net::HttpsConnector;

type Auth = Authorization<Basic>;

struct Event {
    start : DateTime<Local>,
    end : DateTime<Local>,
    title : String,
    url : Option<String>,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} - {} {} {}",
               self.start.format("%F"),
               self.start.format("%R"),
               self.end.format("%R"),
               self.title,
               self.url.as_ref().unwrap_or(&("".to_string())))
    }
}

fn parse_event(v: &serde_json::Value) -> Event {
    let s = v.get("start").unwrap().as_str().unwrap()
             .parse::<DateTime<Local>>().unwrap();
    let e = v.get("end").unwrap().as_str().unwrap()
             .parse::<DateTime<Local>>().unwrap();
    let t = v.get("title").unwrap().as_str().unwrap().to_string();
    let u = v.get("url").unwrap().as_str().map(|s| s.to_string());

    Event {
        start: s,
        end: e,
        title: t,
        url: u,
    }
}

fn get_ev(name: &str) -> Option<String> {
    env::var(name).ok()
}

fn make_auth() -> Option<Auth> {
    let user = get_ev("CAMPH_SCHED_USER");
    let pass =  get_ev("CAMPH_SCHED_PASS");

    user.map(|u| Authorization(Basic {
        username: u,
        password: pass,
    }))
}

fn get_sched(url: String, auth: Option<Auth>) -> Result<Vec<Event>, String> {
    let ssl = hyper_rustls::TlsClient::new();
    let conn = HttpsConnector::new(ssl);
    let client = Client::with_connector(conn);
    let req0 = client.get(&url);
    let req = if let Some(a) = auth { req0.header(a) } else { req0 };

    let mut res = try!(req.send().map_err(|e| e.to_string()));

    if res.status.is_success() {
        let mut res_body = String::new();
        res.read_to_string(&mut res_body).unwrap();
        let vs : serde_json::Value = serde_json::from_str(&res_body).unwrap();
        Ok(vs.as_array().unwrap().iter().map(parse_event).collect())
    } else {
        Err(res.status.to_string())
    }
}

fn main() {
    let url = get_ev("CAMPH_SCHED_URL").expect("Unable to get CAMPH_SCHED_URL");
    let auth = make_auth();

    match get_sched(url, auth) {
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        },
        Ok(es) => {
            let today = Local::today();
            let next = today + chrono::Duration::days(7);

            let mut res : Vec<&Event> = es.iter()
                .filter(|e| e.start.date() >= today && e.end.date() < next)
                .collect();
            &res.sort_by_key(|e| e.start);
            for e in res {
                println!("{}", e);
            }
        },
    }
}
