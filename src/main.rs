use logs::{JournalDLog, Tracer};

mod logs;

fn main() {
    let include_filter = vec!("root");
    let exclude_filter = vec!("session");

    let logs = JournalDLog::new("NetworkManager.service")
        .merge(JournalDLog::new("cron.service"))
        .merge(JournalDLog::new("polkit.service"));


    for line in logs
        .filter(|x| include_filter.iter().all(|y| x.MESSAGE.contains(y)))
        .filter(|x| !exclude_filter.iter().any(|y| x.MESSAGE.contains(y))) {
        println!("{header}\n\t{msg}\n\n",
                 header = line.header(),
                 msg = line.MESSAGE
        );
    }
}
