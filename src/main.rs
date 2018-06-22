#[macro_use]
extern crate nom;

// use std::io;
use std::fs::File;
use std::io::prelude::*;
use nom::{digit1, multispace0, types::CompleteStr};

//==================
// <RULE EXTRACTION>

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

// Consumes until next "
named!(next_quote<CompleteStr,CompleteStr>,
	take_until_and_consume_s!("\"")
);

named!(next_aroba_or_quote<CompleteStr,CompleteStr>,
	take_until_and_consume_s!("\"")
);

// parses one full rule line
named!(read_rule<CompleteStr,(CompleteStr, CompleteStr)>,
	ws!(
		do_parse!(
			name_queue >>
			name: next_quote >>
			next_quote >>
			rule_text: next_aroba_or_quote >>
			white >>
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

// </RULE EXTRACTION>
// ==================

// ==============
// <RULE PARSING>

// Captures integer values
named!(value<CompleteStr,CompleteStr>,
	recognize!(many1!(digit1))
);

// Captures the last half of the range
named!(range_counterpart<CompleteStr,(CompleteStr, CompleteStr)>,
	do_parse!(
		t : tag!("..") >>
		n : value >>
		(t,n)
	)
);

// Captures a numeric range (including singular values)
named!(range<CompleteStr,CompleteStr>,
	recognize!(
		permutation!(
			value,
			opt!(range_counterpart)
		)
	)
);

// Removes comma from desired input
named!(interm_range<CompleteStr,CompleteStr>,
	do_parse!(
		r : range >>
		opt!(tag!(",")) >>
		(r)
	)
);

named!(range_list<CompleteStr,Vec<CompleteStr> >,
   fold_many1!( interm_range, Vec::new(), |mut acc: Vec<_>, item| {
     acc.push(item);
     acc
 }));

// Captures in operators
named!(in_operator<CompleteStr,CompleteStr>,
	alt_complete!(
		tag!("=") |
		tag!("!=") |
		recognize!(
			permutation!(
				opt!(tag!("not")) ,
				alt_complete!(tag!("within") | tag!("in"))
			)
		)
	)
);

// Capture is operators
named!(is_operator<CompleteStr,CompleteStr>,
	recognize!(
		permutation!(
			tag!("is") ,
			opt!(tag!("not"))
		)
	)
);

// Captures an operand
named!(operand<CompleteStr,CompleteStr>,
	alt_complete!(tag!("n") | tag!("i") | tag!("v") | tag!("w") | tag!("f") | tag!("t") )
);

// Captures an expression
named!(expression<CompleteStr,CompleteStr>,
	recognize!(
		permutation!(
			operand ,
			opt!(
				permutation!(
					alt_complete!( tag!("mod") | tag!("%") ) , 
					value
				)
			)
		)
	)
);

named!(in_relation<CompleteStr,(CompleteStr, CompleteStr, Vec<CompleteStr>) >,
	do_parse!(
		first_o : expression >>
		math_o : in_operator >>
		nums : range_list >>
		(first_o, math_o, nums)
	)
);

named!(is_relation<CompleteStr,(CompleteStr, CompleteStr, Vec<CompleteStr>) >,
	do_parse!(
		first_o : expression >>
		math_o : is_operator >>
		nums : range_list >>
		(first_o, math_o, nums)
	)
);


// Extracts plural rule lines for one language
named!(relation<CompleteStr,(CompleteStr, CompleteStr, Vec<CompleteStr>) >,
	alt_complete!(is_relation | in_relation)
);

named!(interm_and_condition<CompleteStr,(CompleteStr, (CompleteStr, CompleteStr, Vec<CompleteStr>))>,
	permutation!(
		tag!("and") ,
		relation
	)
);

named!(and_condition<CompleteStr,((CompleteStr, CompleteStr, Vec<CompleteStr>), Vec<(CompleteStr, (CompleteStr, CompleteStr, Vec<CompleteStr>))>) >,
	do_parse!(
		a : relation >>

		b : fold_many0!( interm_and_condition, Vec::new(), |mut acc: Vec<_>, item| {
		     acc.push(item);
		     acc
		 }) >>
		(a, b)
	)
);

named!(interm_condition<CompleteStr, (CompleteStr, ((CompleteStr, CompleteStr, Vec<CompleteStr>), Vec<(CompleteStr, (CompleteStr, CompleteStr, Vec<CompleteStr>))>)) >,
	permutation!(
		tag!("or") ,
		and_condition
	)
);

named!(condition<CompleteStr, (((CompleteStr, CompleteStr, Vec<CompleteStr>), Vec<(CompleteStr, (CompleteStr, CompleteStr, Vec<CompleteStr>))>), Vec<(CompleteStr, ((CompleteStr, CompleteStr, Vec<CompleteStr>), Vec<(CompleteStr, (CompleteStr, CompleteStr, Vec<CompleteStr>))>))>) >,
	do_parse!(
		a : and_condition >>

		b : fold_many0!( interm_condition, Vec::new(), |mut acc: Vec<_>, item| {
		     acc.push(item);
		     acc
		 }) >>
		(a, b)
	)
);

named!(parse_rule<CompleteStr,(((CompleteStr, CompleteStr, Vec<CompleteStr>), Vec<(CompleteStr, (CompleteStr, CompleteStr, Vec<CompleteStr>))>), Vec<(CompleteStr, ((CompleteStr, CompleteStr, Vec<CompleteStr>), Vec<(CompleteStr, (CompleteStr, CompleteStr, Vec<CompleteStr>))>))>) >,
	call!(condition)
);

// </RULE PARSING>
// ===============

fn main() {
    let filename = "src/plural_en.rule".to_string();

    let mut f = File::open(filename).expect("No such file.");

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
        let rule_name = ((items.1).0).0.to_string();
        let rule_syn = ((items.1).1).0;
        // </GET PLAIN STRINGS FROM RETURN TUPLE>
        // =========================================

        let new_syn = rule_syn.replace(" ", "");

        println!("\nLeftovers:\n================\n{:?}", (items.0).0);

        println!(
            "\nExtracted:\n================\n{:?} :: {:?}",
            rule_name, new_syn
        );

        list_of_rules.push([rule_name, new_syn]);

        println!("Still Working");

        extracted_rules = &items.0;

        if extracted_rules == "" {
            break;
        }
    }
    // </RECURSIVELY EXTRACT NAME:ARGS FROM PLURAL RULES>
    // ===============================================

    // ===============================================
    // <PRINT ALL THE RULE NAME:ARG PAIRS>
    println!("\n================\nRULES:\n================");
    for x in list_of_rules {
        println!("\nLine:\n================\n{:?}", x);

        if x[0] == "other" {
            break;
        }

        let stuff = parse_rule(CompleteStr(&x[1]));

        let items = stuff.unwrap();

        println!("\nInfo:\n{:#?}", items);
    }
    // <PRINT ALL THE RULE NAME:ARG PAIRS>
    // ===============================================
}
