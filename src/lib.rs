extern crate nix;

#[path = "netns_linux.rs"]
mod netns;

pub use netns::NetNS;
