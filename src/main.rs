use logs::{journald, kubectl, Logs, Tracer};

use crate::config::load_config;

mod config;

mod logs;

fn main() {
    let include_filter = vec!();
    let exclude_filter = vec!();
    let conf = load_config("./config/zenbox.yaml").unwrap();

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
