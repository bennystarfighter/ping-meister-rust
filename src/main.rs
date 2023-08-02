pub mod config;
pub mod handlers;

use acidjson::AcidJson;
use chrono::offset::Utc;
use chrono::DateTime;
use clap::Parser;
use core::panic;
use handlers::Response;
use std::time::SystemTime;
use std::{collections::BTreeMap, fs, thread, time};

#[macro_use]
extern crate prettytable;
use prettytable::{color, Attr, Cell, Row, Table};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,

    #[arg(short, long, default_value = "output.json")]
    database_file: String,

    #[arg(short, long)]
    print_output: bool,
}

fn main() {
    let args = Args::parse();
    let dbpath = std::path::Path::new(&args.database_file);
    let json: AcidJson<BTreeMap<String, Response>>;

    match args.print_output {
        // printing operation mode
        true => {
            if !std::path::Path::new(&args.database_file).exists() {
                panic!("The file \"{:}\" does not exist", args.database_file);
            }

            json = match acidjson::AcidJson::open(dbpath) {
                Ok(json) => json,
                Err(error) => panic!("{:}", error),
            };

            let jsonread = json.read();

            println!("[Ping-Meister -----------------]");

            let mut table = Table::new();
            table.add_row(row!["Name", "Status", "Latency"]);

            for response in jsonread.iter() {
                let split: Vec<&str> = response.0.split('|').collect();
                if split.len() != 2 {
                    panic!("Unexpected length of split response read from database")
                }

                let mut new_row: Row = Row::empty();
                new_row.add_cell(Cell::new(split[0]).with_style(Attr::Bold));

                match response.1 {
                    Response::Success { latency } => {
                        new_row.add_cell(
                            Cell::new("Success")
                                .with_style(Attr::BackgroundColor(color::BRIGHT_GREEN)),
                        );
                        new_row.add_cell(Cell::new(latency.to_string().as_str()))
                    }
                    Response::Failure => {
                        new_row.add_cell(
                            Cell::new("Success")
                                .with_style(Attr::BackgroundColor(color::BRIGHT_RED)),
                        );
                        new_row.add_cell(Cell::new((-1).to_string().as_str()))
                    }
                }

                table.add_row(new_row);
            }

            table.printstd();
        }
        // Collecting data operation mode
        false => {
            let conf: config::Config = match config::read_config(args.config) {
                Ok(conf) => conf,
                Err(error) => panic!("{:?}", error),
            };

            if !dbpath.exists() {
                let empty: BTreeMap<String, Response> = BTreeMap::new();
                let out = match serde_json::to_string(&empty) {
                    Ok(json) => json,
                    Err(error) => panic!("{:}", error),
                };

                fs::write(std::path::Path::new(&args.database_file), out).unwrap();
            }

            json = match AcidJson::open(std::path::Path::new(&args.database_file)) {
                Ok(json) => json,
                Err(error) => panic!("Failed to open output file, {:}", error),
            };

            loop {
                let start_time = time::Instant::now();

                let system_time = SystemTime::now();
                let datetime: DateTime<Utc> = system_time.into();
                println!(
                    "Updating {:} targets at {:}.",
                    &conf.targets.len(),
                    datetime.format("%T")
                );

                let mut responses: Vec<(config::Target, Response)> = Vec::new();

                for target in &conf.targets {
                    let resp = target.run_query();
                    responses.push((target.clone(), resp));
                }

                let mut jsonwrite = json.write();

                for response in responses {
                    let key: String = format!("{:}|{:}", response.0.name, response.0.address);
                    jsonwrite.insert(key, response.1);
                }

                let elapsed_time = time::Instant::now().duration_since(start_time);
                let sleep_left = conf.update_interval as i64 - elapsed_time.as_secs() as i64;
                if sleep_left >= 0 {
                    thread::sleep(time::Duration::from_secs(sleep_left as u64));
                };
            }
        }
    }
}
