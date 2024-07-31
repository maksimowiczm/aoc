use std::collections::HashMap;

#[allow(dead_code)]
pub(super) struct System {
    pub workflows: HashMap<String, Workflow>,
    pub parts: Vec<Part>,
}

#[allow(dead_code)]
pub(super) struct Workflow {
    pub rules: Vec<Rule>,
    pub otherwise: ReviewResult,
}

#[allow(dead_code)]
#[derive(Clone)]
pub(super) enum Category {
    Coolness,
    Musicality,
    Aerodynamic,
    Shininess,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Clone)]
pub(super) enum ReviewResult {
    Accept,
    Reject,
    Pass(String),
}

#[allow(dead_code)]
pub(super) struct Part {
    pub coolness: u64,
    pub musicality: u64,
    pub aerodynamic: u64,
    pub shininess: u64,
}
