use std::ffi::c_int;
use std::fs::{File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use nix::errno::Errno;
use nix::sys::signal::{self, SaFlags, SigAction, SigHandler, SigSet, Signal};
use nix::sys::stat::Mode;
use nix::unistd;

pub const DB_FIFO: &str = "/tmp/db_fifo";

pub const NOT_FOUND: &str = "NOTF";

pub const MAX_NAME: usize = 20;

pub const PIPE_BUF: usize = 4096;

pub const MAX_FRAME: usize = PIPE_BUF - 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub id: i32,
    pub name: String,
}

impl Row {
    pub fn new(id: i32, name: &str) -> Self {
        debug_assert!(name.len() <= MAX_NAME, "name is longer than {MAX_NAME}");
        
        Self {
            id,
            name: name.to_owned(),
        }
    }
}

pub fn lookup(db: &[Row], id: i32) -> &str {
    db.iter()
        .find(|&x| x.id == id)
        .map_or(NOT_FOUND, |x| &x.name)
}

pub fn create_fifo(path: &Path) -> io::Result<()> {
    match unistd::mkfifo(path, Mode::from_bits_truncate(0o666)) {
        Ok(()) | Err(Errno::EEXIST) => Ok(()),
        Err(e) => Err(io::Error::from_raw_os_error(e as i32)),
    }
}

pub struct FifoGuard {
    path: PathBuf,
}

impl FifoGuard {
    pub fn create(path: impl Into<PathBuf>) -> io::Result<Self> {
        let path = path.into();
        create_fifo(&path)?;
        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for FifoGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

pub fn open_queue_for_reading(path: &Path) -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open(path)
}

/// `u32 (length) | i32 (id) | path`
pub fn encode_request(id: i32, reply_path: &str) -> Vec<u8> {
    let body_len = 4 + reply_path.len();
    let mut msg = Vec::with_capacity(4 + body_len);
    msg.extend_from_slice(&(body_len as u32).to_ne_bytes());
    msg.extend_from_slice(&id.to_ne_bytes());
    msg.extend_from_slice(reply_path.as_bytes());
    msg
}

pub fn decode_request(body: &[u8]) -> io::Result<(i32, String)> {
    if body.len() <= 4 {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "request without ID or client path",
        ));
    }
    let id = i32::from_ne_bytes(body[..4].try_into().unwrap());
    let path = String::from_utf8(body[4..].to_vec())
        .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
    
    Ok((id, path))
}

/// `u32 (lenght) | answer`
pub fn encode_response(answer: &str) -> Vec<u8> {
    let mut msg = Vec::with_capacity(4 + answer.len());
    msg.extend_from_slice(&(answer.len() as u32).to_ne_bytes());
    msg.extend_from_slice(answer.as_bytes());
    msg
}

pub fn decode_response(body: &[u8]) -> io::Result<String> {
    String::from_utf8(body.to_vec())
        .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))
}

pub fn read_frame(queue: &mut File) -> io::Result<Vec<u8>> {
    let mut len_buf = [0u8; 4];
    read_full(queue, &mut len_buf)?;

    let len = u32::from_ne_bytes(len_buf) as usize;
    if len == 0 || len > MAX_FRAME {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!("incorrect frame lenght: {len}"),
        ));
    }

    let mut body = vec![0u8; len];
    read_full(queue, &mut body)?;
    Ok(body)
}

fn read_full(queue: &mut File, buf: &mut [u8]) -> io::Result<()> {
    let mut got = 0;
    while got < buf.len() {
        match queue.read(&mut buf[got..]) {
            Ok(0) => return Err(ErrorKind::UnexpectedEof.into()),
            Ok(n) => got += n,
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

pub fn send_frame(path: &Path, frame: &[u8]) -> io::Result<()> {
    if frame.len() > PIPE_BUF {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "komunikat dłuższy niż PIPE_BUF — zapis przestałby być niepodzielny",
        ));
    }

    let mut fifo = OpenOptions::new().write(true).open(path)?;
    let written = fifo.write(frame)?;
    if written != frame.len() {
        return Err(io::Error::new(
            ErrorKind::WriteZero,
            format!("zapis rozbity na części: {written}/{}", frame.len()),
        ));
    }
    Ok(())
}



static STOP: AtomicBool = AtomicBool::new(false);

/// Jedyne, co wolno robić w obsłudze sygnału — ustawić flagę.
extern "C" fn on_sigusr1(_: c_int) {
    STOP.store(true, Ordering::SeqCst);
}

pub fn install_server_handlers() -> io::Result<()> {
    let ignore = SigAction::new(SigHandler::SigIgn, SaFlags::empty(), SigSet::empty());
    let stop = SigAction::new(
        SigHandler::Handler(on_sigusr1),
        SaFlags::empty(),
        SigSet::empty(),
    );

    unsafe {
        signal::sigaction(Signal::SIGHUP, &ignore)
            .and_then(|_| signal::sigaction(Signal::SIGTERM, &ignore))
            .and_then(|_| signal::sigaction(Signal::SIGUSR1, &stop))
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;
    }
    Ok(())
}

pub fn stop_requested() -> bool {
    STOP.load(Ordering::SeqCst)
}
