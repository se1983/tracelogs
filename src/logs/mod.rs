pub use journald::JournalDLog;
pub(crate) use lib::Logs;
pub use lib::Tracer;

mod lib;
mod journald;

