use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "session_scripts", about = "Run scripts on session signals")]
pub struct SessionScriptsConfig {
    /// Command to run when the session is locked
    #[structopt(long)]
    pub lock: Option<String>,

    /// Command to run when the session is unlocked
    #[structopt(long)]
    pub unlock: Option<String>,
}

pub fn parse_args() -> SessionScriptsConfig {
    SessionScriptsConfig::from_args()
}
