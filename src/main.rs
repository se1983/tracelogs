use logs::{journald, kubectl, Logs, Tracer};

use clap::Clap;

use crate::config::load_config;
use std::process::exit;

mod config;
mod logs;


#[derive(Clap)]
#[clap(version = "0.1", author = "Sebastian S. <sebsch@geblubber.org>")]
struct Opts {
    #[clap(short, long, default_value = "./config/zenbox.yaml")]
    config_file: String,
    #[clap(short, long,  multiple = true)]
    exclude_filter: Option<Vec<String>>,
    #[clap(short, long, multiple = true)]
    include_filter: Option<Vec<String>>,
}


fn main() {

    let opts: Opts = Opts::parse();

    let include_filter = opts.include_filter.unwrap_or(vec!());
    let exclude_filter = opts.exclude_filter.unwrap_or(vec!());
    let conf = load_config(&opts.config_file).unwrap_or_else(|err| {
        println!("[{}]", err);
            exit(2);
    });

    let mut logs = Logs::new(vec!());
    for t in kubectl::build_logs(&conf) {
        logs = logs.merge(Logs::from(t));
    }
    for t in journald::build_logs(&conf) {
        logs = logs.merge(Logs::from(t));
    }


    for line in logs.filter_logs(&exclude_filter, &include_filter) {
        line.print_line();
    }
}
