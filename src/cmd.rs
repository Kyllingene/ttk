use std::io::{BufReader, Read, Write};
use std::process::{Command, Stdio};

pub fn run<A: AsRef<[S]>, S: AsRef<str> + AsRef<std::ffi::OsStr>>(
    level: u32,
    cmd: String,
    args: A,
) -> bool {
    let args = args.as_ref();

    if args.is_empty() {
        crate::print::info!(level, "Running command `{cmd}`",);

        crate::logfile::write_line(&format!("=== Running command `{cmd}`",));
    } else {
        let args_display = crate::display::from_iter(
            args.iter().map(|arg| {
                crate::display::from_fn(move |f| {
                    let arg: &str = arg.as_ref();
                    if arg
                        .chars()
                        .any(|ch| !ch.is_ascii_graphic() || ['\'', '"'].contains(&ch))
                    {
                        write!(f, "{arg:?}")
                    } else {
                        write!(f, "{arg}")
                    }
                })
            }),
            "",
        );

        crate::print::info!(level, "Running command `{cmd} {args_display}`",);

        crate::logfile::write_line(&format!("=== Running command `{cmd} {args_display}`",));
    }

    let mut command = Command::new(cmd);
    command.args(args);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            crate::print::err!(level, "Failed to run command: {e}");
            return false;
        }
    };

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let mut logfile = crate::logfile::handle();
    let mut child_stdout = BufReader::new(stdout);
    let mut child_stderr = BufReader::new(stderr);
    let mut buf = [0; 1024];
    let mut stdout = std::io::stdout().lock();

    let status = loop {
        let read = match child_stdout.read(&mut buf) {
            Ok(read) => read,
            Err(e) => {
                crate::print::warn!(level, "Failed to read command stdout: {e}");
                0
            }
        };

        if read > 0 {
            if let Err(e) = stdout.write_all(&buf[..read]) {
                crate::print::err!(level, "Failed to write to stdout: {e}");
            }

            if let Some(logfile) = logfile.as_mut()
                && let Err(e) = logfile.write_all(&buf[..read])
            {
                crate::print::warn!(level, "Failed to write to logfile: {e}");
            }
        }

        let read = match child_stderr.read(&mut buf) {
            Ok(read) => read,
            Err(e) => {
                crate::print::warn!(level, "Failed to read command stderr: {e}");
                0
            }
        };

        if read > 0 {
            if let Err(e) = stdout.write_all(&buf[..read]) {
                crate::print::err!(level, "Failed to write to stdout: {e}");
            }

            if let Some(logfile) = logfile.as_mut()
                && let Err(e) = logfile.write_all(&buf[..read])
            {
                crate::print::warn!(level, "Failed to write to logfile: {e}");
            }
        }

        match child.try_wait() {
            Ok(Some(status)) => break status.success(),
            Ok(None) => {}
            Err(e) => {
                crate::print::err!(level, "Failed to finish command: {e}");
                break false;
            }
        }
    };

    let read = match child_stdout.read(&mut buf) {
        Ok(read) => read,
        Err(e) => {
            crate::print::warn!(level, "Failed to read command stdout: {e}");
            0
        }
    };

    if read > 0 {
        if let Err(e) = stdout.write_all(&buf[..read]) {
            crate::print::err!(level, "Failed to write to stdout: {e}");
        }

        if let Some(logfile) = logfile.as_mut()
            && let Err(e) = logfile.write_all(&buf[..read])
        {
            crate::print::warn!(level, "Failed to write to logfile: {e}");
        }
    }

    let read = match child_stderr.read(&mut buf) {
        Ok(read) => read,
        Err(e) => {
            crate::print::warn!(level, "Failed to read command stderr: {e}");
            0
        }
    };

    if read > 0 {
        if let Err(e) = stdout.write_all(&buf[..read]) {
            crate::print::err!(level, "Failed to write to stdout: {e}");
        }

        if let Some(logfile) = logfile.as_mut()
            && let Err(e) = logfile.write_all(&buf[..read])
        {
            crate::print::warn!(level, "Failed to write to logfile: {e}");
        }
    }

    status
}

pub fn run_silent<A: AsRef<[S]>, S: AsRef<str> + AsRef<std::ffi::OsStr>>(
    level: u32,
    cmd: String,
    args: A,
) -> bool {
    let args = args.as_ref();

    if args.is_empty() {
        crate::print::info!(level, "Running command `{cmd}`",);

        crate::logfile::write_line(&format!("=== Running command `{cmd}`",));
    } else {
        let args_display = crate::display::from_iter(
            args.iter().map(|arg| {
                crate::display::from_fn(move |f| {
                    let arg: &str = arg.as_ref();
                    if arg
                        .chars()
                        .any(|ch| !ch.is_ascii_graphic() || ['\'', '"'].contains(&ch))
                    {
                        write!(f, "{arg:?}")
                    } else {
                        write!(f, "{arg}")
                    }
                })
            }),
            "",
        );

        crate::print::info!(level, "Running command `{cmd} {args_display}`",);

        crate::logfile::write_line(&format!("=== Running command `{cmd} {args_display}`",));
    }

    let mut command = Command::new(cmd);
    command.args(args);

    if let Some(logfile) = crate::logfile::handle()
        && let Ok(handle_1) = logfile.try_clone()
        && let Ok(handle_2) = logfile.try_clone()
    {
        command.stdout(Stdio::from(handle_1));
        command.stderr(Stdio::from(handle_2));
    } else {
        // either:
        // - this line ran because there is no logfile
        // - this line ran because we failed to clone the logfile's handle
        // we want this in the latter case. in the former case, this works out to a no-op.
        crate::logfile::write_line("Failed to write command output to logfile");

        command.stdout(Stdio::null());
        command.stderr(Stdio::null());
    }
    command.stdin(Stdio::null());

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            crate::print::err!(level, "Failed to run command: {e}");
            return false;
        }
    };

    match child.wait() {
        Ok(status) => status.success(),
        Err(e) => {
            crate::print::err!(level, "Failed to finish command: {e}");
            false
        }
    }
}

#[macro_export]
macro_rules! cmd {
    ($level:expr, silent $cmd:expr $(,)?) => {
        $crate::cmd::run_silent::<_, &str>($level, $cmd.into(), &[]);
    };

    ($level:expr, $cmd:expr $(,)?) => {
        $crate::cmd::run::<_, &str>($level, $cmd.into(), &[]);
    };

    ($level:expr, silent $cmd:expr, $($args:expr),* $(,)?) => {
        $crate::cmd::run_silent($level, $cmd.into(), &[$(::std::string::String::from($args)),*]);
    };

    ($level:expr, $cmd:expr, $($args:expr),* $(,)?) => {
        $crate::cmd::run($level, $cmd.into(), &[$(::std::string::String::from($args)),*]);
    };
}

pub use crate::cmd as run;
