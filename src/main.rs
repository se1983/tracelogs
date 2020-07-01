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

    for line in logs
        .filter(|x| include_filter.iter().all(|y| x.message.contains(y)))
        .filter(|x| !exclude_filter.iter().any(|y| x.message.contains(y))) {
        println!("{header}\n\t{msg}\n\n",
                 header = line.header(),
                 msg = line.message
        );
    }
}
