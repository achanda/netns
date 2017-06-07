extern crate netns;

use std::process::Command;
use netns::NetNS;

fn main() {
	let ns = NetNS::new();
	if ns.is_ok() {
		let temp = ns.unwrap();
		println!("{:?}", temp);
		NetNS::set(temp);
		let output = Command::new("ip").arg("a").output().unwrap();
		println!("{}", String::from_utf8_lossy(&output.stdout));
		println!("{}", String::from_utf8_lossy(&output.stderr));
	}
}
