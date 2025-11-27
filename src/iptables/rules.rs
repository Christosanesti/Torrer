// iptables rule definitions and utilities

/// iptables rule types
#[derive(Debug, Clone)]
pub enum RuleType {
    Nat,
    Filter,
    Mangle,
}

/// iptables rule
#[derive(Debug, Clone)]
pub struct Rule {
    pub table: RuleType,
    pub chain: String,
    pub rule: Vec<String>,
}

