#[macro_use]
extern crate nom;

// use std::io;
use std::fs::File;
use std::io::prelude::*;

// use nom::{alphanumeric1, space, types::CompleteStr};
use nom::{types::CompleteStr, multispace0};

// Consumes all the content leading up to language marker (only english)
named!(eat_head<CompleteStr,CompleteStr>,
	take_until_and_consume!("\"en\": {")
);

// Consumes until not whitespace (allows for no whitespace)
named!(white<CompleteStr,CompleteStr>,
	call!(multispace0)
);

// Consumes up to plural rule true name
named!(name_queue<CompleteStr,CompleteStr>,
	take_until_and_consume_s!("\"pluralRule-count-") 
);

named!(next_aroba<CompleteStr,CompleteStr>,
	take_until_and_consume_s!("@")
);

// Consumes until next "
named!(next_quote<CompleteStr,CompleteStr>,
	take_until_and_consume_s!("\"")
);

named!(next_aroba_or_quote<CompleteStr,CompleteStr>,
	do_parse!(
		end : next_aroba >>
		next_quote >>
		(end)
	)
);

// parses one full rule line
named!(read_rule<CompleteStr,(CompleteStr, CompleteStr)>,
	ws!(
		do_parse!(
			white >>
			name_queue >>
			name: next_quote >>
			next_quote >>
			rule_text: next_aroba_or_quote >>
			(name, rule_text)
		)
	)
);

// consumes until next }
named!(eat_rules<CompleteStr,CompleteStr>,
	take_until_and_consume_s!("}")
);

// Extracts plural rule lines for one language
named!(get_rules<CompleteStr,CompleteStr>,
	do_parse!(
		eat_head >>
		all_rules : eat_rules >>
		(all_rules)
	)
);

fn main() {

	let filename = "src/plural_en.rule".to_string();

	let mut f = File::open(filename)
		.expect("No such file.");

	let mut contents = String::new();

	f.read_to_string(&mut contents)
		.expect("Something went wrong reading file.");

	let desired_text = get_rules(CompleteStr(&contents));

	let mut extracted_rules = (desired_text.unwrap().1).0;

	let mut list_of_rules = Vec::new();

	// ===============================================
	// <RECURSIVELY EXTRACT NAME:ARGS FROM PLURAL RULES>
	loop {

		let temp = CompleteStr(&extracted_rules);

		let stuff = read_rule(temp);

		let items = stuff.unwrap();

		// =========================================
		// <GET PLAIN STRINGS FROM RETURN TUPLE>
		let rule_name = ((items.1).0).0;
		let rule_syn = ((items.1).1).0;
		// </GET PLAIN STRINGS FROM RETURN TUPLE>
		// =========================================

		println!("\nLeftovers:\n================\n\"{:?}\"", items.0);

		println!("\nExtracted:\n================\n\"{:?}\" :: \"{:?}\"", rule_name, rule_syn);

		list_of_rules.push([rule_name, rule_syn]);

		extracted_rules = &items.0;		

		if extracted_rules == "" {
			break
		}
	}
	// </RECURSIVELY EXTRACT NAME:ARGS FROM PLURAL RULES>
	// ===============================================

	// ===============================================
	// <PRINT ALL THE RULE NAME:ARG PAIRS>
	println!("\nRULES:\n================");
	for x in list_of_rules {
		println!("{:?}", x);
	}
	// <PRINT ALL THE RULE NAME:ARG PAIRS>
	// ===============================================
}