use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static LOGFILE: OnceLock<File> = OnceLock::new();

pub fn init(name: impl AsRef<Path>) {
    let path = new_path(name);

    if let Some(dir) = path.parent()
        && let Err(e) = std::fs::create_dir_all(dir)
    {
        crate::err!(1, "Failed to create logfile at {}: {e}", path.display());
        return;
    }

    let file = match File::create(&path) {
        Ok(f) => f,
        Err(e) => {
            crate::err!(1, "Failed to create logfile at {}: {e}", path.display());
            return;
        }
    };

    if LOGFILE.set(file).is_err() {
        crate::warn!(1, "Tried to initialize logfile twice");
    }

    crate::dbg!(1, "Logfile created at {}", path.display());
}

pub fn new_path(name: impl AsRef<Path>) -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| ".".into());
    path.push(name);

    let id = rand::random::<u64>();
    path.push(format!("log-{id}.txt"));

    path
}

pub fn handle() -> Option<&'static File> {
    LOGFILE.get()
}

pub fn write_line(line: &str) {
    let Some(mut file) = handle() else { return };

    if let Err(e) = file.write_all(line.as_bytes()) {
        crate::warn!(1, "Failed to write to logfile: {e}");
    }

    if !line.ends_with('\n')
        && let Err(e) = file.write_all(b"\n")
    {
        crate::warn!(1, "Failed to write to logfile: {e}");
    }
}
