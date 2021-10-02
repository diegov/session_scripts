use dbus::arg;
use dbus::arg::ReadAll;
use dbus::blocking::Connection;
use dbus::channel::Token;
use dbus::message::MatchRule;
use dbus::strings::{BusName, Member, Path};
use dbus::Message;
use dbus::MessageType;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;

struct Signal {}

impl ReadAll for Signal {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(Signal {})
    }
}

const DESTINATION: &str = "org.freedesktop.login1";
const INTERFACE: &str = "org.freedesktop.login1.Session";

pub struct Service {
    path: String,
    conn: Connection,
    listeners: Vec<Token>,
}

impl Service {
    pub fn new(session_id: &str) -> Result<Service, dbus::Error> {
        let conn = Connection::new_system()?;
        Ok(Service {
            path: format!("/org/freedesktop/login1/session/{}", session_id),
            conn,
            listeners: Vec::new(),
        })
    }

    pub fn add_signal_listener(
        &mut self,
        command: &Option<String>,
        signal: &str,
    ) -> Result<(), dbus::Error> {
        if let Some(command_value) = command {
            let mut m: MatchRule = Default::default();
            let sender: BusName = DESTINATION.into();

            m.sender = Some(sender);
            let path: Path = self.path.to_string().into();
            m.path = Some(path.into_static());
            m.msg_type = Some(MessageType::Signal);
            m.interface = Some(INTERFACE.into());
            let member: Member = signal.into();
            m.member = Some(member.into_static());

            let command_clone = command_value.clone();
            let id: dbus::channel::Token =
                self.conn
                    .add_match(m, move |_: Signal, _: &Connection, _: &Message| {
                        let mut command = Command::new(&command_clone);
                        command.stdout(Stdio::inherit());
                        command.stderr(Stdio::inherit());
                        // TODO: Log instead
                        let _output = command.output().expect("Failed to execute command");
                        true
                    })?;

            self.listeners.push(id);
        }

        Ok(())
    }

    pub fn process(&self) -> Result<bool, dbus::Error> {
        self.conn.process(Duration::from_millis(2000))
    }
}
