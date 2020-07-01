use logs::{JournalDLog, Logs, Tracer, LogingSchemes};
use crate::logs::{RegExtractor, KubectlLog};

mod logs;

fn main() {
    let include_filter = vec!();
    let exclude_filter = vec!();
    let ssh_host = "ssh://KEPPLER.nextcloud";

    let datetime = r"[0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1]) (2[0-3]|[01][0-9]):[0-5][0-9]:[0-5][0-9],[0-9][0-9][0-9]";
    let host = r"([^\s]+)";
    let service = r"([^\s]+)";
    let message = r"(.*)";
    let log_pattern = "(?P<datetime>({d})) (?P<hostname>({h}))] (?P<service>({s})) (?P<message>({m}))";
    let strftime_pattern = "%Y-%m-%d %H:%M:%S,%f";


    let scheme = LogingSchemes {
        date_time: datetime.to_string(),
        host: host.to_string(),
        service: service.to_string(),
        message: message.to_string(),
        whole_line: log_pattern.to_string()
    };

    let extractor = RegExtractor::new(scheme, strftime_pattern);
    let pod = "testrunner";


    let mut logs = Logs::from(KubectlLog::new(pod, extractor));
    logs = logs.merge(Logs::from(JournalDLog::new("cron.service", Some(&ssh_host))));


    for line in logs.filter_logs(&exclude_filter, &include_filter) {
        line.print_line();
    }
}
