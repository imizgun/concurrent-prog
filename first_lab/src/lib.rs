use std::{fs, io, process, thread, time::Duration};

pub const DATA_FILE: &str = "dane";
pub const RESULT_FILE: &str = "wyniki";

pub const INTERVAL: Duration = Duration::from_millis(10);

pub fn write_atomic(path: &str, contents: &str) -> io::Result<()> {
    let tmp = format!("{path}.tmp.{}", process::id());
    fs::write(&tmp, contents)?;
    fs::rename(&tmp, path)
}

pub fn wait_and_take(path: &str) -> io::Result<String> {
    let claimed = format!("{path}.{}", process::id());
    loop {
        match fs::rename(path, &claimed) {
            Ok(()) => break,
            Err(e) if e.kind() == io::ErrorKind::NotFound => thread::sleep(INTERVAL),
            Err(e) => return Err(e),
        }
    }
    let contents = fs::read_to_string(&claimed)?;
    fs::remove_file(&claimed)?;
    Ok(contents)
}

pub fn remove_if_exists(path: &str) -> io::Result<()> {
    match fs::remove_file(path) {
        Err(e) if e.kind() != io::ErrorKind::NotFound => Err(e),
        _ => Ok(()),
    }
}
