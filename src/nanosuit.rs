//! Nanosuit Mode System - Crysis-inspired adaptive analysis modes
//! Modes: Strength (deep forensics), Speed (real-time optimization),
//! Armor (quantum-resistant signing), Stealth (covert monitoring)

use serde::{Deserialize, Serialize};
use std::fmt;

/// Nanosuit operational modes for adaptive code analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NanosuitMode {
    /// Deep forensic scans with eBPF tracing at kernel-level
    Strength,
    /// Real-time gas optimization for Solidity contracts
    Speed,
    /// Quantum-resistant signing with Dilithium2
    Armor,
    /// Covert IoT monitoring without detection
    Stealth,
}

impl NanosuitMode {
    /// Get energy cost multiplier for this mode
    pub fn energy_cost(&self) -> f64 {
        match self {
            NanosuitMode::Strength => 2.5,  // High CPU for deep scans
            NanosuitMode::Speed => 1.0,     // Standard real-time
            NanosuitMode::Armor => 3.0,     // Expensive crypto ops
            NanosuitMode::Stealth => 0.5,   // Low power covert mode
        }
    }

    /// Check if mode can detect quantum threats
    pub fn quantum_aware(&self) -> bool {
        matches!(self, NanosuitMode::Armor | NanosuitMode::Strength)
    }

    /// Get recommended analysis depth
    pub fn analysis_depth(&self) -> usize {
        match self {
            NanosuitMode::Strength => 10,  // Deep AST traversal
            NanosuitMode::Speed => 3,      // Shallow quick scan
            NanosuitMode::Armor => 7,      // Crypto-focused depth
            NanosuitMode::Stealth => 5,    // Medium depth
        }
    }
}

impl fmt::Display for NanosuitMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NanosuitMode::Strength => write!(f, "STRENGTH [eBPF Forensics]"),
            NanosuitMode::Speed => write!(f, "SPEED [Gas Optimization]"),
            NanosuitMode::Armor => write!(f, "ARMOR [Quantum Shield]"),
            NanosuitMode::Stealth => write!(f, "STEALTH [Covert Monitor]"),
        }
    }
}

/// Nanosuit analyzer with mode switching
pub struct NanosuitAnalyzer {
    current_mode: NanosuitMode,
    energy_level: f64,
}

impl NanosuitAnalyzer {
    pub fn new() -> Self {
        Self {
            current_mode: NanosuitMode::Speed,
            energy_level: 100.0,
        }
    }

    /// Switch to a different operational mode
    pub fn switch_mode(&mut self, mode: NanosuitMode) {
        log::info!("Switching from {} to {}", self.current_mode, mode);
        self.current_mode = mode;
    }

    /// Consume energy based on current mode
    pub fn consume_energy(&mut self, duration_secs: f64) {
        let cost = self.current_mode.energy_cost() * duration_secs;
        self.energy_level = (self.energy_level - cost).max(0.0);
    }

    /// Check if enough energy for operation
    pub fn has_energy(&self) -> bool {
        self.energy_level > 10.0
    }

    /// Get current mode
    pub fn mode(&self) -> NanosuitMode {
        self.current_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_energy() {
        assert_eq!(NanosuitMode::Strength.energy_cost(), 2.5);
        assert_eq!(NanosuitMode::Stealth.energy_cost(), 0.5);
    }

    #[test]
    fn test_quantum_awareness() {
        assert!(NanosuitMode::Armor.quantum_aware());
        assert!(!NanosuitMode::Speed.quantum_aware());
    }

    #[test]
    fn test_analyzer_switching() {
        let mut analyzer = NanosuitAnalyzer::new();
        assert_eq!(analyzer.mode(), NanosuitMode::Speed);
        
        analyzer.switch_mode(NanosuitMode::Armor);
        assert_eq!(analyzer.mode(), NanosuitMode::Armor);
    }
}
