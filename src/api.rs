use std::io::Read;
use serde_json;
use config::Config;
use reqwest::header::{Headers, Authorization, Basic, Accept};
use chrono::{Utc, DateTime};
use chrono::format::ParseError;
use std::io::BufReader;
use std::io::BufRead;
use reqwest::RequestBuilder;
use reqwest::Client;

pub fn remove_ack(conf: Config, filter: &str) {
    let url = format!("https://{}:{}/v1/actions/remove-acknowledgement?{}",
                      conf.server,
                      conf.port,
                      filter);
    let client = post_client(conf, url);
    match client.send() {
        Ok(mut resp) => {
            let mut body = vec![];
            resp.read_to_end(&mut body).unwrap();
            println!("{}", String::from_utf8_lossy(&body));
        }
        Err(e) => println!("http post failed: {}", e),
    }
}

pub fn remove_downtime(conf: Config, name: &str) {
    let url = format!("https://{}:{}/v1/actions/remove-downtime?downtime={}",
                      conf.server,
                      conf.port,
                      name);
    let client = post_client(conf, url);
    match client.send() {
        Ok(mut resp) => {
            let mut body = vec![];
            resp.read_to_end(&mut body).unwrap();
            println!("{}", String::from_utf8_lossy(&body));
        }
        Err(e) => panic!("Failed to reach icinga! {}", e),
    }
}

pub fn remove_comment(conf: Config, name: &str) {
    let url = format!("https://{}:{}/v1/actions/remove-comment?comment={}",
                      conf.server,
                      conf.port,
                      name);
    let client = post_client(conf, url);
    match client.send() {
        Ok(mut resp) => {
            let mut body = vec![];
            resp.read_to_end(&mut body).unwrap();
            println!("{}", String::from_utf8_lossy(&body));
        }
        Err(e) => println!("http post failed: {}", e),
    }
}

pub fn shutdown_icinga(conf: Config) {
    let url = format!("https://{}:{}/v1/actions/shutdown-process",
                      conf.server,
                      conf.port);
    let client = post_client(conf, url);
    match client.send() {
        Ok(mut resp) => {
            let mut body = vec![];
            resp.read_to_end(&mut body).unwrap();
            println!("{}", String::from_utf8_lossy(&body));
        }
        Err(e) => println!("http post failed: {}", e),
    }
}

pub fn restart_icinga(conf: Config) {
    let url = format!("https://{}:{}/v1/actions/restart-process",
                      conf.server,
                      conf.port);
    let client = post_client(conf, url);
    match client.send() {
        Ok(mut resp) => {
            let mut body = vec![];
            resp.read_to_end(&mut body).unwrap();
            println!("{}", String::from_utf8_lossy(&body));
        }
        Err(e) => println!("http post failed: {}", e),
    }
}

pub fn stream(conf: Config, filter: &str) {
    let url = format!("https://{}:{}/v1/events?queue=icingacli{}",
                      conf.server,
                      conf.port,
                      filter);
    let client = post_client(conf, url);
    match client.send() {
        Ok(mut resp) => {
            let buf = BufReader::new(&mut resp);
            for line in buf.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(e) => println!("http post failed: {}", e),
    }
}

pub fn date_parse(input: &str) -> Result<DateTime<Utc>, ParseError> {
    input.parse::<DateTime<Utc>>()
}

pub fn icinga_problems(conf: Config, filter: &str) {
    let url = format!("https://{}:{}/v1/objects/services?attrs=name&attrs=state&filter=service.state==Service{}",
                      conf.server,
                      conf.port,
                      filter);
    let client = get_client(conf, url);
    match client.send() {
        Ok(mut resp) => {
            let mut body = vec![];
            resp.read_to_end(&mut body).unwrap();
            println!("{}", String::from_utf8_lossy(&body));
        }
        Err(e) => println!("http get failed: {}", e),
    };
}

pub fn icinga_status(conf: Config, filter: String) {
    let url = format!("https://{}:{}/v1/status/{}", conf.server, conf.port, filter);
    let client = get_client(conf, url);
    match client.send() {
        Ok(mut resp) => {
            let mut body = vec![];
            resp.read_to_end(&mut body).unwrap();
            println!("{}", String::from_utf8_lossy(&body));
        }
        Err(e) => println!("Failed to get icinga {}", e),
    };
}

fn get_client(conf: Config, url: String) -> RequestBuilder {
    let c = Client::new().unwrap();
    let mut headers = Headers::new();
    headers.set(Authorization(Basic {
                                  username: conf.user,
                                  password: conf.password,
                              }));
    headers.set(Accept::json());
    c.get(url.as_str()).headers(headers)
}

fn post_client(conf: Config, url: String) -> RequestBuilder {
    let c = Client::new().unwrap();
    let mut headers = Headers::new();
    headers.set(Authorization(Basic {
                                  username: conf.user,
                                  password: conf.password,
                              }));
    headers.set(Accept::json());
    c.post(url.as_str()).headers(headers)
}

pub struct Group {
    name: String,
}

impl Group {
    pub fn new(name: String) -> Group {
        Group { name: name }
    }

    pub fn send(&self, conf: Config) {
        let url = format!("https://{}:{}/v1/objects/hosts?filter=\"{}\" in host.groups",
                          conf.server,
                          conf.port,
                          &self.name);
        let client = get_client(conf, url);
        match client.send() {
            Ok(mut resp) => {
                let mut body = vec![];
                resp.read_to_end(&mut body).unwrap();
                println!("{}", String::from_utf8_lossy(&body));
            }
            Err(e) => println!("http get failed: {}", e),
        };

    }
}

pub struct Host {
    name: String,
}

impl Host {
    pub fn new(name: String) -> Host {
        Host { name: name }
    }

    pub fn send(&self, conf: Config) {
        let url = format!("https://{}:{}/v1/objects/hosts?host={}",
                          conf.server,
                          conf.port,
                          &self.name);
        let client = get_client(conf, url);
        match client.send() {
            Ok(mut resp) => {
                let mut body = vec![];
                resp.read_to_end(&mut body).unwrap();
                println!("{}", String::from_utf8_lossy(&body));
            }
            Err(e) => println!("http get failed: {}", e),
        };

    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddComment {
    pub author: String,
    pub comment: String,
}

impl AddComment {
    pub fn new(author: String, comment: String) -> AddComment {
        AddComment {
            author: author,
            comment: comment,
        }
    }

    pub fn send(&self, conf: Config, filter: String) {
        let url = format!("https://{}:{}/v1/actions/add-comment{}",
                          conf.server,
                          conf.port,
                          filter);
        let client = post_client(conf, url);
        let payload = serde_json::to_string(&self).unwrap();
        match client.body(payload.as_str()).send() {
            Ok(mut resp) => {
                let mut body = vec![];
                resp.read_to_end(&mut body).unwrap();
                println!("{}", String::from_utf8_lossy(&body));
            }
            Err(e) => println!("http post failed: {}", e),
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateTicket {
    pub cn: String,
}

impl GenerateTicket {
    pub fn new(cn: String) -> GenerateTicket {
        GenerateTicket { cn: cn }
    }

    pub fn send(&self, conf: Config) {
        let url = format!("https://{}:{}/v1/actions/generate-ticket",
                          conf.server,
                          conf.port);
        let client = post_client(conf, url);
        let payload = serde_json::to_string(&self).unwrap();
        match client.body(payload.as_str()).send() {
            Ok(mut resp) => {
                let mut body = vec![];
                resp.read_to_end(&mut body).unwrap();
                println!("{}", String::from_utf8_lossy(&body));
            }
            Err(e) => println!("http post failed: {}", e),
        };

    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AcknowledgeProblem {
    author: String,
    comment: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiry: Option<i64>,
    sticky: bool,
    notify: bool,
    #[serde(skip_serializing)]
    filter: String,
}

impl AcknowledgeProblem {
    pub fn new(author: String,
               comment: String,
               expiry: Option<i64>,
               sticky: bool,
               notify: bool,
               filter: String)
               -> AcknowledgeProblem {
        AcknowledgeProblem {
            author: author,
            comment: comment,
            expiry: expiry,
            sticky: sticky,
            notify: notify,
            filter: filter,
        }
    }

    pub fn send(&self, conf: Config) {
        let url = format!("https://{}:{}/v1/actions/acknowledge-problem?{}",
                          conf.server,
                          conf.port,
                          &self.filter);
        let client = post_client(conf, url);
        let payload = serde_json::to_string(&self).unwrap();
        match client.body(payload.as_str()).send() {
            Ok(mut resp) => {
                let mut body = vec![];
                resp.read_to_end(&mut body).unwrap();
                println!("{}", String::from_utf8_lossy(&body));
            }
            Err(e) => println!("http post failed: {}", e),
        };

    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScheduleDowntime {
    author: String,
    comment: String,
    start_time: i64,
    end_time: i64,
    child_options: i64,
}

impl ScheduleDowntime {
    pub fn new(author: String,
               comment: String,
               start_time: i64,
               end_time: i64,
               child_options: i64)
               -> ScheduleDowntime {
        ScheduleDowntime {
            author: author,
            comment: comment,
            start_time: start_time,
            end_time: end_time,
            child_options: child_options,
        }
    }

    pub fn send(&self, conf: Config, filter: String) {
        let url = format!("https://{}:{}/v1/actions/schedule-downtime{}",
                          conf.server,
                          conf.port,
                          filter);
        let client = post_client(conf, url);
        let payload = serde_json::to_string(&self).unwrap();
        match client.body(payload.as_str()).send() {
            Ok(mut resp) => {
                let mut body = vec![];
                resp.read_to_end(&mut body).unwrap();
                println!("{}", String::from_utf8_lossy(&body));
            }
            Err(e) => println!("http post failed: {}", e),
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RescheduleCheck {
    next_check: i64,
    force_check: bool,
    _type: String,
    filter: String,
}

impl RescheduleCheck {
    pub fn send(&self, conf: Config) {
        let url = format!("https://{}:{}/v1/actions/reschedule-check",
                          conf.server,
                          conf.port);
        let client = post_client(conf, url);
        let payload = serde_json::to_string(&self).unwrap();
        match client.body(payload.as_str()).send() {
            Ok(mut resp) => {
                let mut body = vec![];
                resp.read_to_end(&mut body).unwrap();
                println!("{}", String::from_utf8_lossy(&body));
            }
            Err(e) => println!("http post failed: {}", e),
        };
    }
}
