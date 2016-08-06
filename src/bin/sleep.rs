//! XSI-compliant implementation of `sleep`.
//! Version 0.1.1
//! [Manpage](http://pubs.opengroup.org/onlinepubs/9699919799/utilities/sleep.html)

fn main() {
	use std::{env, time, thread};

	let mut args = env::args();

	if args.len() < 2 {
		println!("{}: Missing operand.", args.next().unwrap());
		std::process::exit(1);
	} else if args.len() > 2 {
		println!("{}: Too many operands.", args.next().unwrap());
		std::process::exit(1);
	} else {// args.len() == 2
		match args.nth(1).unwrap().parse::<u64>() {
			Ok(sec) => {
				let d = time::Duration::new(sec, 0);
				thread::sleep(d);
			},
			Err(err) => {
				println!("{}: Invalid time interval. It should be a positive decimal integer.", args.next().unwrap());
				std::process::exit(1);
			},
		}
	}
}
