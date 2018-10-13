extern crate icingacli;
#[macro_use]
extern crate clap;
extern crate chrono;

use icingacli::config::Config;
use icingacli::config::DEFAULT_PATH;
use std::path::Path;
use clap::App;
use icingacli::api;
use std::env;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();
    let path = if matches.is_present("config") {
        matches.value_of("config").unwrap().to_string()
    } else {
        format!("{}/{}",
                env::var_os("HOME").unwrap().to_str().unwrap(),
                DEFAULT_PATH)
    };
    let conf: Config = match Config::read_config(Path::new(path.as_str())) {
        Ok(c) => c,
        Err(e) => panic!("failed to parse toml cfg file, {}", e),
    };
    match matches.subcommand_name() {
        Some("schedule-downtime") => {
            let matches = matches.subcommand_matches("schedule-downtime").unwrap();
            let author = matches
                .value_of("author")
                .expect("requires a author")
                .to_string();
            let comment = matches
                .value_of("comment")
                .expect("requires a comment")
                .to_string();
            let st = matches
                .value_of("start-time")
                .expect("requires a start_time");
            let start_time = api::date_parse(st)
                .expect("not a valid date provided!")
                .timestamp();
            let et = matches.value_of("end-time").expect("requires a end_time");
            let end_time = api::date_parse(et)
                .expect("not a valid date provided!")
                .timestamp();
            let child_options = if let true = matches.is_present("child_options") {
                matches
                    .value_of("child_options")
                    .unwrap()
                    .parse::<i64>()
                    .unwrap()
            } else {
                0
            };
            let sdt =
                api::ScheduleDowntime::new(author, comment, start_time, end_time, child_options);
            if matches.is_present("service") & matches.is_present("host") {
                let filter = format!("?service={}!{}",
                                     matches.value_of("host").unwrap(),
                                     matches.value_of("service").unwrap());
                sdt.send(conf, filter);
            } else if matches.is_present("service") {
                let filter = format!("?service={}", matches.value_of("service").unwrap());
                sdt.send(conf, filter);
            } else if matches.is_present("host") {
                let filter = format!("?host={}", matches.value_of("host").unwrap());
                sdt.send(conf, filter);
            }
        }
        Some("shutdown") => {
            api::shutdown_icinga(conf);
        }
        Some("add-comment") => {
            let matches = matches.subcommand_matches("add-comment").unwrap();
            if matches.is_present("author") & matches.is_present("comment") {
                let author = matches.value_of("author").unwrap().to_string();
                let comment = matches.value_of("comment").unwrap().to_string();
                let comment = api::AddComment::new(author, comment);
                if matches.is_present("service") & matches.is_present("host") {
                    let filter = format!("?service={}!{}",
                                         matches.value_of("host").unwrap(),
                                         matches.value_of("service").unwrap());
                    comment.send(conf, filter);
                } else if matches.is_present("service") {
                    let filter = format!("?service={}", matches.value_of("service").unwrap());
                    comment.send(conf, filter);
                } else if matches.is_present("host") {
                    let filter = format!("?host={}", matches.value_of("host").unwrap());
                    comment.send(conf, filter);
                } else {
                    println!("Must provide host or service");
                }
            }
        }
        Some("remove-acknowledgement") => {
            let matches = matches
                .subcommand_matches("remove-acknowledgement")
                .unwrap();
            if matches.is_present("service") & matches.is_present("host") {
                let filter = format!("?service={}!{}",
                                     matches.value_of("host").unwrap(),
                                     matches.value_of("service").unwrap());
                api::remove_ack(conf, filter.as_str());
            } else if matches.is_present("service") {
                let filter = format!("?service={}", matches.value_of("service").unwrap());
                api::remove_ack(conf, filter.as_str());
            } else if matches.is_present("host") {
                let filter = format!("?host={}", matches.value_of("host").unwrap());
                api::remove_ack(conf, filter.as_str());
            }
        }
        Some("remove-comment") => {
            let matches = matches.subcommand_matches("remove-comment").unwrap();
            if matches.is_present("name") {
                let name = matches.value_of("name").unwrap();
                api::remove_comment(conf, name);
            }
        }
        Some("remove-downtime") => {
            let matches = matches.subcommand_matches("remove-downtime").unwrap();
            if matches.is_present("name") {
                let name = matches.value_of("name").unwrap();
                api::remove_downtime(conf, name);
            }
        }
        Some("generate-ticket") => {
            let matches = matches.subcommand_matches("generate-ticket").unwrap();
            if matches.is_present("cn") {
                let cn = matches.value_of("cn").unwrap().to_string();
                let cn = api::GenerateTicket { cn: cn };
                cn.send(conf)
            } else {
                println!("Requires a common-name to generate ticket!")
            };
        }
        Some("restart") => {
            icingacli::api::restart_icinga(conf);
        }
        Some("problems") => {
            let matches = matches.subcommand_matches("problems").unwrap();
            if matches.args.is_empty() {
                api::icinga_problems(conf.clone(), "Critical%7c%7cservice.state==ServiceWarning%7c%7cservice.state==ServiceUnknown")
            } else {
                for state in &matches.args {
                    api::icinga_problems(conf.clone(), state.0)
                }
            }
        }
        Some("status") => {
            let matches = matches.subcommand_matches("status").unwrap();
            if matches.args.is_empty() {
                api::icinga_status(conf, "".to_string())
            } else {
                for status in &matches.args {
                    api::icinga_status(conf.clone(), status.0.to_string())
                }
            }
        }
        Some("group") => {
            let matches = matches.subcommand_matches("group").unwrap();
            let group = match matches.value_of("group") {
                Some(group) => group,
                None => panic!("requires a host"),
            };
            let g = api::Group::new(group.to_string());
            g.send(conf);
        }
        Some("host") => {
            let matches = matches.subcommand_matches("host").unwrap();
            let host = match matches.value_of("host") {
                Some(host) => host,
                None => panic!("requires a host"),
            };
            let h = api::Host::new(host.to_string());
            h.send(conf);
        }
        Some("acknowledge-problem") => {
            let matches = matches.subcommand_matches("acknowledge-problem").unwrap();
            let author = match matches.value_of("author") {
                Some(author) => author,
                None => panic!("requires a author"),
            };
            let comment = match matches.value_of("comment") {
                Some(comment) => comment,
                None => panic!("requires a comment"),
            };
            let sticky = if let true = matches.is_present("sticky") {
                true
            } else {
                false
            };
            let notify = if let true = matches.is_present("notify") {
                true
            } else {
                false
            };
            let expiry = if matches.is_present("expiry") {
                match api::date_parse(matches.value_of("expiry").unwrap()) {
                    Ok(d) => d.timestamp(),
                    Err(e) => {
                        println!("err");
                        panic!("Error parsing your date entry. Please enter a valid date! {}",
                               e)
                    }
                }
            } else {
                0
            };
            let filter = if matches.is_present("service") & matches.is_present("host") {
                format!("service={}!{}",
                        matches.value_of("host").unwrap(),
                        matches.value_of("service").unwrap())
            } else if matches.is_present("service") {
                format!("service={}", matches.value_of("service").unwrap())
            } else if matches.is_present("host") {
                format!("host={}", matches.value_of("host").unwrap())
            } else {
                panic!("host or service required!")
            };
            let ack_struct = if matches.is_present("expiry") {
                api::AcknowledgeProblem::new(author.to_string(),
                                             comment.to_string(),
                                             Some(expiry),
                                             sticky,
                                             notify,
                                             filter)
            } else {
                api::AcknowledgeProblem::new(author.to_string(),
                                             comment.to_string(),
                                             None,
                                             sticky,
                                             notify,
                                             filter)
            };
            println!("{:?}", ack_struct);
            ack_struct.send(conf);
        }
        Some("stream") => {
            let matches = matches.subcommand_matches("stream").unwrap();
            let mut filter = String::new();
            for f in &matches.args {
                filter.push_str(format!("&types={}", f.0).as_str());
            }
            if matches.is_present("filter") {
                filter.push_str(format!("&filter={}", matches.value_of("filter").unwrap()).as_str())
            }
            api::stream(conf, &filter);
        }
        _ => println!("Not a valid sub command"),
    }
}
