use structopt::StructOpt;

/// Program arguments
#[derive(StructOpt)]
pub struct Opts {
    /// Rom location
    #[structopt(short, long)]
    pub rom: Option<String>,

    /// Steps per clock cycle.
    #[structopt(short, long, default_value = "1")]
    pub clock: usize,

    /// Disable sound
    #[structopt(long = "nosound")]
    pub no_sound: bool,

    /// Sound frequency
    #[structopt(short = "f", long = "freq", default_value = "500")]
    pub beep_freq: u32,
}
