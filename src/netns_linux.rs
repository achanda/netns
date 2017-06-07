extern crate nix;

use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use nix::sched::{unshare, setns};
use nix::fcntl::open;
use nix::unistd::{getpid, gettid};
use nix::sys::stat::Mode;
use nix::Error as NError;

pub struct NetNS {
    fd: i32,
    path: PathBuf,
}

#[derive(Debug)]
pub enum NetNSError {
    CreateNetNSError,
}

impl Error for NetNSError {
    fn description(&self) -> &str {
        "Cannot create"
    }
}

impl fmt::Debug for NetNS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NetNS {{ fd: {}, path: {} }}", self.fd, self.path.display())
    }
}

impl NetNS {
    pub fn new() -> Result<NetNS, NetNSError> {
        unshare(nix::sched::CLONE_NEWNET).expect("failed");
        NetNS::get()
    }

    pub fn get() -> Result<NetNS, NetNSError> {
        return NetNS::get_from_thread(getpid(), gettid());
    }

    pub fn set(ns: NetNS) -> Result<(), NError> {
        setns(ns.fd, nix::sched::CLONE_NEWNET)
    }

    fn get_from_thread(pid: i32, tid: i32) -> Result<NetNS, NetNSError> {
        return NetNS::get_from_path(PathBuf::from(format!("/proc/{}/task/{}/ns/net", pid, tid)
            .as_str()));
    }

    fn get_from_path(path: PathBuf) -> Result<NetNS, NetNSError> {
        let fd = open(&path, nix::fcntl::O_RDONLY, Mode::empty()).expect("Could not open");
        return Ok(NetNS {
            fd: fd,
            path: path,
        });
    }
}

impl fmt::Display for NetNSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "oops")
    }
}
