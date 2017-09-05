//! XSI-compliant implementation of `pwd`.
//! Version 0.0.3
//! [Manpage](http://pubs.opengroup.org/onlinepubs/9699919799/utilities/pwd.html)

extern crate getopts;
//using getopt is probably not the best idea for this tool
//but I wanted to learn it
//TODO: remove getopt?
fn main () {
	//use getopts;
	use std::{env, path, process};

	let args: Vec<String> = std::env::args().skip(1).collect();

	let mut opts = getopts::Options::new();
	opts.optflag("L", "", "Logical path");
	opts.optflag("P", "", "Physical path");

	let matches = match opts.parse(&args) {
		Ok(m) => m,
		Err(err) => panic!(err.to_string())
	};

	let cwd = match env::current_dir() {
		Ok(m) => m,
		Err(err) => panic!(err.to_string())
	};

	if matches.opt_present("P") && if args.len() >= 2 { args[1].chars().last() == Some('P') } else { true } {
		println!("{}", cwd.display());
	} else {//no option or -L
		match env::var("PWD") {
			Ok(val) => {
				//PWD value validation
				let env_path = path::Path::new(&val);
				match env::set_current_dir(&env_path) {
					Ok(()) => {//could change 

						();

						let new_cwd = match env::current_dir() {
							Ok(m) => m,
							Err(f) => panic!(f.to_string())
						};

						if new_cwd == cwd {
							println!("{}", val);
						} else {
							println!("{}", cwd.display());//getcwd and $PWD don't match
						}
					},
					Err(_) => println!("{}", cwd.display())//couldn't change directory to $PWD
				}

			},
			Err(err) => {
				eprintln!("{}", err);//$PWD has no value
				process::exit(1);
			},
		}
	}
}
