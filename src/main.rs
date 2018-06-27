#[macro_use]
extern crate nom;
#[macro_use]
extern crate serde_derive;

mod ast;
mod parser;

use nom::digit;
use nom::types::CompleteStr;
use std::str::FromStr;

#[derive(Debug)]
pub struct Value(pub usize);

#[derive(Debug)]
pub struct Range {
    pub lower: Value,
    pub upper: Value,
}

#[derive(Debug)]
pub enum RangeListItem {
  Value(Value),
  Range(Range),
}

#[derive(Debug)]
pub struct RangeList(Vec<RangeListItem>);

named!(value <CompleteStr, Value>, do_parse!(
  v: map!(digit, |i| usize::from_str(&i).unwrap()) >>
  (Value(v))
));

named!(range<CompleteStr, Range>,
	do_parse!(
        value_low: value >>
        is_a!("..") >>
        value_high: value >>
        (Range {
            lower: value_low,
            upper: value_high
        })
	)
);

named!(range_or_value<CompleteStr, RangeListItem>, alt!(
  range   => { |r| RangeListItem::Range(r) } |
  value   => { |v| RangeListItem::Value(v) }
));

named!(range_list<CompleteStr, RangeList>, do_parse!(
  values: separated_list!(ws!(tag!(",")), range_or_value) >>
  (RangeList(values))
));

fn main() {
    let _stuff = range_list(CompleteStr("5..6 , 4..0"));
    // println!("\nInfo:\n{:#?}", stuff);


	let u = parser::parse_plurals_resource("plurals.json").unwrap();

    let en_rules = u.supplemental.rules.get("en").unwrap();
    let en_rule_one = en_rules.get("pluralRule-count-one").unwrap();

    println!("{:#?}", en_rules);
    println!("{:#?}", en_rule_one);
}
