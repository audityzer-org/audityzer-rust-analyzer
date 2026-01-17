pub mod reentrancy;

use crate::analyzer::Vulnerability;

pub use reentrancy::ReentrancyDetector;

pub struct OverflowDetector;
pub struct AccessControlDetector;

impl OverflowDetector {
    pub fn new() -> Self {
        Self
    }
}

impl AccessControlDetector {
    pub fn new() -> Self {
        Self
    }
}

impl crate::analyzer::VulnerabilityDetector for OverflowDetector {
    fn name(&self) -> &str {
        "overflow-detector"
    }

    fn detect(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Vulnerability> {
        Vec::new() // TODO: Implement overflow detection
    }
}

impl crate::analyzer::VulnerabilityDetector for AccessControlDetector {
    fn name(&self) -> &str {
        "access-control-detector"
    }

    fn detect(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Vulnerability> {
        Vec::new() // TODO: Implement access control detection
    }
}
