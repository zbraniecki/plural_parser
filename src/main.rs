#[macro_use]
extern crate nom;

// use std::io;
use std::fs::File;
use std::io::prelude::*;

// use nom::{alphanumeric1, space, types::CompleteStr};
use nom::{types::CompleteStr};

named!(eat_head<CompleteStr,CompleteStr>,
	take_until_s!("supplemental\": {")
);

named!(read_rules<CompleteStr,CompleteStr>,
	ws!(
		do_parse!(
			head: eat_head >>
			(head)
		)

	)
);

// +---------- Old test code ----------+
// fn get_rule(rule: String) -> String {
// 	rule
// }

fn main() {
	// +---------- Old test code ----------+
	// let line = "Grammar Rule".to_string();
	// let updated = get_rule(line);
	// println!("{}", updated);

	let filename = "src/plural_en.rule".to_string();

	let mut f = File::open(filename)
		.expect("No such file.");

	let mut contents = String::new();

	f.read_to_string(&mut contents)
		.expect("Something went wrong reading file.");

	// println!("{}", &contents);

	let stuff = read_rules(CompleteStr(&contents));

	println!("{:?}", stuff);
}
