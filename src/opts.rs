use structopt::StructOpt;

/// Program arguments
#[derive(StructOpt)]
pub struct Opts {
    /// Disable sound
    #[structopt(long = "nosound")]
    pub no_sound: bool,

    /// Rom location
    #[structopt(short, long)]
    pub rom: Option<String>,

    /// Sound frequency
    #[structopt(short = "f", long = "freq")]
    pub beep_freq: Option<u32>,
}
