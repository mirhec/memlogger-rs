use sysinfo::{self, ProcessExt, SystemExt, Process};
use clap::{App, Arg};
use std::u32;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use timer;
use chrono;

#[macro_use] extern crate quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum CliError {
        ParseInt(err: std::num::ParseIntError) { from() }
        Io(err: std::io::Error) { from() }
    }
}

fn main() -> Result<(), CliError> {
    let matches = App::new("memlogger")
                            .version("1.0")
                            .author("Mirko Hecky <m.hecky@simon-ibv.de>")
                            .about("Memlogger enables you to log the memory and cpu consumption of one ore more processes into csv files.")
                            .arg(Arg::with_name("interval")
                                .short("i")
                                .long("interval")
                                .value_name("INTERVAL")
                                .default_value("1")
                                .help("Sets the interval (in seconds) in which the memory is gathered")
                                .required(false)
                                .takes_value(true))
                            .arg(Arg::with_name("output")
                                .short("o")
                                .long("output")
                                .value_name("FILE")
                                .help("Path to the output file [default = memlogger_[datetime].csv")
                                .required(false)
                                .takes_value(true))
                            .arg(Arg::with_name("processes")
                                .short("p")
                                .value_name("PROCESS1,PROCESS2,...")
                                .value_delimiter(",")
                                .multiple(true)
                                .min_values(1)
                                .required(true)
                                .takes_value(true))
                            .get_matches();

    let interval: u32 = matches.value_of("interval").unwrap().parse()?;
    let output = match matches.value_of("output") {
        Some(val) => val.to_owned(),
        None => format!("memlogger_{}.csv", chrono::Local::now().format("%Y-%m-%d %H-%M-%S"))
    };

    // Print headers
    File::create(&output)?.write("Timestamp;PID;Name;Status;Memory (kB);CPU\n".as_bytes())?;

    // Start timer
    let mut system = sysinfo::System::new();
    let mut file = OpenOptions::new().append(true).open(&output)?;
    let timer = timer::Timer::new();
    let _guard = timer.schedule_repeating(chrono::Duration::seconds(interval as i64), move || {
        system.refresh_processes();
        let process_names = matches.values_of("processes").unwrap();
        let processes = system.get_process_list();
        let mut list: Vec<&Process> = Vec::new();
        for name in process_names {
            for (_pid, p) in processes {
                if p.name().to_lowercase().starts_with(name.to_lowercase().as_str()) {
                    list.push(p);
                    break;
                }
            }
        }

        for p in list {
            writeln!(file, "{};{};{};{};{};{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), p.pid(), p.name(), p.status(), p.memory(), p.cpu_usage()).unwrap();
        }
    });

    loop { std::thread::sleep(std::time::Duration::from_secs(1)); }
}
