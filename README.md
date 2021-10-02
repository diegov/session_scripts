# What is this

A lightweight program that can run commands on DBus session signals. 

The equivalent of piping `dbus-monitor` to a loop, but instead of a shell script, it's a rust program. And it only supports a very small set of signals.

I use it to clear cached ssh and gpg credentials when the session is locked.

# Usage

```
USAGE:
    session_scripts [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --lock <lock>        Command to run when the session is locked
        --unlock <unlock>    Command to run when the session is unlocked
```

# TODO

- Figure out how to get rid of the `XDG_SESSION_ID` requirement. There is a `/org/freedesktop/login1/session/self` session that might point to the current session instead of needing to put `XDG_SESSION_ID` in the DBus path.
- More signals

# License

GPLv3, see [LICENSE](./LICENSE).
