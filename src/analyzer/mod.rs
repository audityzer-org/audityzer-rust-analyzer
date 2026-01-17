use tree_sitter::{Parser, Tree};
use std::collections::HashMap;

pub mod detectors;
pub use detectors::*;

pub struct VulnerabilityAnalyzer {
    parser: Parser,
    detectors: Vec<Box<dyn VulnerabilityDetector>>,
}

pub trait VulnerabilityDetector: Send + Sync {
    fn name(&self) -> &str;
    fn detect(&self, tree: &Tree, source: &str) -> Vec<Vulnerability>;
}

#[derive(Debug, Clone)]
pub struct Vulnerability {
    pub severity: Severity,
    pub title: String,
    pub description: String,
    pub line: usize,
    pub column: usize,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl VulnerabilityAnalyzer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut parser = Parser::new();
        let language = tree_sitter_solidity::language();
        parser.set_language(language)?;

        let detectors: Vec<Box<dyn VulnerabilityDetector>> = vec![
            Box::new(detectors::ReentrancyDetector::new()),
            Box::new(detectors::OverflowDetector::new()),
            Box::new(detectors::AccessControlDetector::new()),
        ];

        Ok(Self { parser, detectors })
    }

    pub fn analyze(&mut self, source: &str) -> Result<AnalysisReport, Box<dyn std::error::Error>> {
        let tree = self.parser.parse(source, None)
            .ok_or("Failed to parse source code")?;

        let mut all_vulnerabilities = Vec::new();

        for detector in &self.detectors {
            let vulns = detector.detect(&tree, source);
            all_vulnerabilities.extend(vulns);
        }

        all_vulnerabilities.sort_by(|a, b| {
            b.severity.cmp(&a.severity)
                .then_with(|| a.line.cmp(&b.line))
        });

        Ok(AnalysisReport {
            vulnerabilities: all_vulnerabilities,
            total_lines: source.lines().count(),
            detectors_run: self.detectors.len(),
        })
    }
}

#[derive(Debug)]
pub struct AnalysisReport {
    pub vulnerabilities: Vec<Vulnerability>,
    pub total_lines: usize,
    pub detectors_run: usize,
}

impl PartialOrd for Severity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Severity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_val = match self {
            Severity::Critical => 4,
            Severity::High => 3,
            Severity::Medium => 2,
            Severity::Low => 1,
            Severity::Info => 0,
        };
        let other_val = match other {
            Severity::Critical => 4,
            Severity::High => 3,
            Severity::Medium => 2,
            Severity::Low => 1,
            Severity::Info => 0,
        };
        self_val.cmp(&other_val)
    }
}

impl Eq for Severity {}
