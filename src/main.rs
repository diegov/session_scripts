use std::env;

mod config;
mod signals;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = config::parse_args();
    let session_id = env::var("XDG_SESSION_ID").expect("Failed to get XDG_SESSION_ID");

    let mut service = signals::Service::new(&session_id).expect("Failed to initialise DBus");

    service.add_signal_listener(&conf.lock, "Lock")?;
    service.add_signal_listener(&conf.unlock, "Unlock")?;

    loop {
        service.process()?;
    }
}
