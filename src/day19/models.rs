use std::collections::HashMap;

pub(super) struct System {
    pub workflows: HashMap<String, Workflow>,
    pub parts: Vec<Part>,
}

pub(super) struct Workflow {
    pub rules: Vec<Rule>,
    pub otherwise: ReviewResult,
}

#[derive(Clone)]
pub(super) enum Category {
    Coolness,
    Musicality,
    Aerodynamic,
    Shininess,
}

#[derive(Clone)]
pub(super) enum Rule {
    LessThan {
        category: Category,
        value: u64,
        then: ReviewResult,
    },
    GreaterThan {
        category: Category,
        value: u64,
        then: ReviewResult,
    },
}

#[derive(Clone)]
pub(super) enum ReviewResult {
    Accept,
    Reject,
    Pass(String),
}

pub(super) struct Part {
    pub coolness: u64,
    pub musicality: u64,
    pub aerodynamic: u64,
    pub shininess: u64,
}
