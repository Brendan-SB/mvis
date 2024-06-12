mod config;
mod display;
mod play;
mod run;

use run::run;

pub const PROGRAM_NAME: &str = "mvis";
pub const PROGRAM_DESC: &str = "A command line music visualizer.";
pub const NO_HOME: &str = "No home folder";
pub const IMPROPER_HEX_FORMAT: &str = "Improper hex format";
pub const FROM_OS_STRING_FAILED: &str = "Failed to change OsString to String";

fn main() {
    if let Err(e) = run() {
        println!("{e}");
    }
}
