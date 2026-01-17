use tree_sitter::{Tree, Node, Query, QueryCursor};
use crate::analyzer::{VulnerabilityDetector, Vulnerability, Severity};

pub struct ReentrancyDetector;

impl ReentrancyDetector {
    pub fn new() -> Self {
        Self
    }

    fn check_external_call(&self, node: &Node, source: &str) -> bool {
        let text = &source[node.byte_range()];
        text.contains(".call{") || text.contains(".transfer(") || text.contains(".send(")
    }

    fn check_state_change_after_call(&self, node: &Node, source: &str) -> bool {
        let mut cursor = node.walk();
        let mut found_call = false;
        
        for child in node.children(&mut cursor) {
            if self.check_external_call(&child, source) {
                found_call = true;
            } else if found_call && child.kind() == "assignment_expression" {
                return true;
            }
        }
        false
    }
}

impl VulnerabilityDetector for ReentrancyDetector {
    fn name(&self) -> &str {
        "reentrancy-detector"
    }

    fn detect(&self, tree: &Tree, source: &str) -> Vec<Vulnerability> {
        let mut vulnerabilities = Vec::new();
        let root = tree.root_node();
        
        let mut cursor = root.walk();
        self.traverse_node(&root, source, &mut vulnerabilities, &mut cursor);
        
        vulnerabilities
    }
}

impl ReentrancyDetector {
    fn traverse_node(
        &self,
        node: &Node,
        source: &str,
        vulns: &mut Vec<Vulnerability>,
        cursor: &mut tree_sitter::TreeCursor,
    ) {
        if node.kind() == "function_definition" {
            if self.check_state_change_after_call(node, source) {
                let pos = node.start_position();
                vulns.push(Vulnerability {
                    severity: Severity::Critical,
                    title: "Reentrancy Vulnerability".to_string(),
                    description: "State change after external call detected".to_string(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    suggestion: Some("Use checks-effects-interactions pattern".to_string()),
                });
            }
        }

        for child in node.children(cursor) {
            self.traverse_node(&child, source, vulns, cursor);
        }
    }
}
