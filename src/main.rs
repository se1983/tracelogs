use logs::{JournalDLog, Logs, Tracer};

mod logs;

fn main() {
    let include_filter = vec!("root");
    let exclude_filter = vec!("session");
    let host = "ssh://KEPPLER.nextcloud";


    let mut logs = Logs::new_from(JournalDLog::new("NetworkManager.service", None));
    logs.merge(
        Logs::new_from(JournalDLog::new("Cron.service", Some(&host)))
    );

    for line in logs.filter_logs(&exclude_filter, &include_filter) {
        line.print_line();
    }
}
