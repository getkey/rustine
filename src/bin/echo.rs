//! XSI-compliant implementation of `echo`.
//! Version 0.2.0
//! [Manpage](http://pubs.opengroup.org/onlinepubs/9699919799/utilities/echo.html)

fn main() {
	use std::env;

	let mut args = env::args().skip(1);

	if let Some(arg) = args.next() {
		print_arg(arg); // no space in front of first arg
	}
	for arg in args {
		print!(" ");
		print_arg(arg);
	}

	print!("\n");
}

#[derive(Copy, Clone)]
enum States {
	Start,
	Slash,
	CollectNumOne,
	CollectNumTwo(u8),
	CollectNumThree(u8),
}

fn print_arg(arg: String) {
	use std::process;
	use States::*;

	let mut state = Start;

	for curr_char in arg.chars() {
		match (state, curr_char) {
			(Start, '\\') => state = Slash,
			(Start, _) => print!("{}", curr_char),

			(Slash, '\\') => {
				print!("{}", '\\');
				state = Start;
			},
			(Slash, 'a') => {
				print!("{}", 7 as char);
				state = Start;
			},
			(Slash, 'b') => {
				print!("{}", 8 as char);
				state = Start;
			},
			(Slash, 'c') => process::exit(0),
			(Slash, 'f') => {
				print!("{}", 12 as char);
				state = Start;
			},
			(Slash, 'n') => {
				print!("{}", '\n');
				state = Start;
			},
			(Slash, 'r') => {
				print!("{}", '\r');
				state = Start;
			},
			(Slash, 't') => {
				print!("{}", '\t');
				state = Start;
			},
			(Slash, 'v') => {
				print!("{}", 11 as char);
				state = Start;
			},
			(Slash, '0') => state = CollectNumOne,
			(Slash, _) => {
				print!("\\{}", curr_char);
				state = Start;
			}

			(CollectNumOne, '0'...'7') => state = CollectNumTwo(curr_char.to_digit(8).unwrap() as u8),
			(CollectNumOne, _) => state = Start,


			(CollectNumTwo(num), '0'...'7') => state = CollectNumThree(num*8 + curr_char.to_digit(8).unwrap() as u8),
			(CollectNumTwo(num), _) => {
				print!("{}", num as char);
				state = Start
			},

			(CollectNumThree(num), '0'...'7') => {
				if let Some(num) = num.checked_mul(8) {
					if let Some(num) = num.checked_add(curr_char.to_digit(8).unwrap() as u8) {
						print!("{}", num as char);
					}
				}
				state = Start;
			},
			(CollectNumThree(num), _) => {
				print!("{}", num as char);
				state = Start;
			},
		}
	}
}
