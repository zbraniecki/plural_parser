// Structs for CLDR rules

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub conditions: Vec<AndCondition>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AndCondition {
    pub relations: Vec<Relation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Relation {
    pub expression: Expression,
    pub operator: Operator,
    pub range_list: RangeList,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operator {
    pub operator: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub operand: Operand,
    pub modulo_operator: Option<Modulo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Modulo {
    pub modulus: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operand {
    pub operand: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangeList {
    pub range_list: Vec<RangeListItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RangeListItem{
    Range(Range),
    Value(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub lower_val: Value,
    pub upper_val: Value
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub val: isize,
}
