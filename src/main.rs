#[macro_use]
extern crate nom;

mod ast;

// use std::io;
use std::fs::File;
use std::io::prelude::*;
use nom::{digit1, multispace0, types::CompleteStr};

use ast::*;

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
named!(value<CompleteStr, Value>,
	map!(recognize!(many1!(digit1)), |recast| Value { val: recast.to_string() } )
);

// Captures the last half of the range
named!(range_counterpart<CompleteStr,(String, Value)>,
	do_parse!(
		t : map!(tag!(".."), |recast| recast.to_string() ) >>
		n : value >>
		(t,n)
	)
);

// Captures a numeric range (including singular values)
named!(range<CompleteStr,Range>,
	map!(
		recognize!(
			permutation!(
				value,
				opt!(range_counterpart)
			)
		)
	, |recast| Range { range: recast.to_string() } )
);

// Removes comma from desired input
named!(interm_range<CompleteStr,Range>,
	do_parse!(
		r : range >>
		opt!(tag!(",")) >>
		(r)
	)
);

named!(range_list<CompleteStr, RangeList >,
   map!(
		fold_many1!( interm_range, Vec::new(), |mut acc: Vec<_>, item| {
		acc.push(item);
		acc
		}), |recast| RangeList { range_list : recast} )
);

// Captures in operators
named!(in_operator<CompleteStr,Operator>,
	map!(
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
	, |recast| Operator { operator : recast.to_string() } )
);

// Capture is operators
named!(is_operator<CompleteStr,Operator>,
	map!(
		recognize!(
			permutation!(
				tag!("is") ,
				opt!(tag!("not"))
			)
		)
	, |recast| Operator { operator: recast.to_string() } ) 
);

// Captures an operand
named!(operand<CompleteStr,Operand>,
	map!(
		alt_complete!(
			tag!("n") | 
			tag!("i") | 
			tag!("v") | 
			tag!("w") | 
			tag!("f") | 
			tag!("t") ), 
		|recast| Operand { operand : recast.to_string() } )
);

named!(mod_expression<CompleteStr,Modulo>,
	do_parse!(
		t: alt_complete!( tag!("mod") | tag!("%") ) >>
		v : value >>
		(Modulo {
			modulus: t.to_string() ,
			value : v
		})
	)
);

// Captures an expression
named!(expression<CompleteStr,Expression>,
	do_parse!(
		rand: operand >>
		mod_expr: opt!(mod_expression) >>
		(Expression { 
			operand: rand,
			modulo_operator: mod_expr
		})
	)
);

named!(in_relation<CompleteStr, Relation >,
	do_parse!(
		first_o : expression >>
		math_o : in_operator >>
		nums : range_list >>
		(Relation{
			expression: first_o, 
			operator: math_o, 
			range_list: nums
		})
	)
);

named!(is_relation<CompleteStr,Relation >,
	do_parse!(
		first_o : expression >>
		math_o : is_operator >>
		nums : range_list >>
		( Relation {
			expression: first_o, 
			operator: math_o, 
			range_list: nums
		})
	)
);


// Extracts plural rule lines for one language
named!(relation<CompleteStr, Relation >,
	alt_complete!(is_relation | in_relation)
);

named!(and_relation<CompleteStr,Relation >,
	do_parse!(
		opt!(tag!("and")) >>
		r: relation >>
		(r)
	)
);

named!(and_condition<CompleteStr,AndCondition >,
	do_parse!(
		a : fold_many0!( and_relation, Vec::new(), |mut acc: Vec<_>, item| {
		     acc.push(item);
		     acc
		 }) >>
		(AndCondition{
			relations: a
		})
	)
);

named!(interm_condition<CompleteStr, AndCondition >,
	do_parse!(
		opt!(tag!("or")) >>
		s: and_condition >>
		(s)
	)
);

named!(condition<CompleteStr, Condition >,
	do_parse!(
		a : fold_many0!( interm_condition, Vec::new(), |mut acc: Vec<_>, item| {
		     acc.push(item);
		     acc
		 }) >>
		(Condition{
			conditions: a
		})
	)
);

named!(parse_rule<CompleteStr,Condition >,
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
