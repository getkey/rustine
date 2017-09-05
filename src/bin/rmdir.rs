//! XSI-compliant implementation of `rmdir`.
//! Version 0.0.0
//! [Manpage](http://pubs.opengroup.org/onlinepubs/9699919799/utilities/rmdir.html)

#[derive(Copy, Clone)]
enum States {
	Start,
	Iterate,
	IteratePathname,
}

fn main() {
	use std::{env, process, path};
	use States::*;

	let mut args = env::args().skip(1);

	if args.len() == 0 {
		eprintln!("{}: Missing operand.", args.nth(0).unwrap());
		process::exit(1);
	}

	let mut state = Start;

	for arg in args {
		let arg_slice = &arg[..];
		match (state, arg_slice) {
			(Start, "-p") => state = IteratePathname,
			(Start, _) => {
				rmdir(path::Path::new(arg_slice));
				state = Iterate
			},

			(Iterate, _) => rmdir(path::Path::new(arg_slice)),
			(IteratePathname, _) => rmdir_pathname(arg_slice),
		}
	}
}

fn rmdir(dir: &std::path::Path) {
	use std::{fs, process};

	if let Err(err) = fs::remove_dir(dir) {
		eprintln!("rmdir: failed to remove \"{}\": {}", dir.display(), err);
		process::exit(1);
	}
}

fn rmdir_pathname(dir: &str) {
	use std::path;

	let mut dir_path = path::Path::new(dir);

	loop {
		rmdir(dir_path);
		match dir_path.parent() {
			Some(parent) => if parent.as_os_str().is_empty() {
				break;
			} else {
				dir_path = parent
			},
			None => break,
		}
	}
}
