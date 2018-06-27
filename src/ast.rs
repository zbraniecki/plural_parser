// Structs for CLDR rules

#[derive(Debug, Clone, PartialEq)]
pub struct Cond {
    pub condition_a: AndCond,
    pub condition_b: Vec<OrCond>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrCond {
    pub or_condition_a: String,
    pub or_condition_b: AndCond,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AndCond {
    pub and_condition_a: Rel,
    pub and_condition_b: Vec<AndRel>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AndRel {
    pub and_rel_a: String,
    pub and_rel_b: Rel,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rel {
    pub expression: Exp,
    pub operator: String,
    pub range_list: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exp {
    pub operand: String,
    pub modulo_operator: Option<ModExp>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModExp {
    pub modulo: String,
    pub value: String,
}
