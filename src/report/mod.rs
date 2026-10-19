pub mod csv_export;

use crate::metrics::FertilityMetrics;
use crate::types::{LanguageCode, OverheadThreshold};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FlaggedLanguage {
    pub tokenizer: String,
    pub language: String,
    pub fertility: f64,
    pub overhead: f64,
}

pub struct ReportGenerator;

impl ReportGenerator {
    /// Rank languages by fertility per tokenizer (highest = worst overhead)
    pub fn rank_by_fertility(metrics: &[FertilityMetrics]) -> HashMap<String, Vec<FertilityMetrics>> {
        let mut map: HashMap<String, Vec<FertilityMetrics>> = HashMap::new();
        for m in metrics {
            map.entry(m.tokenizer.clone()).or_default().push(m.clone());
        }
        for v in map.values_mut() {
            // Sort descending fertility
            v.sort_by(|a, b| b.fertility.partial_cmp(&a.fertility).unwrap_or(std::cmp::Ordering::Equal));
        }
        map
    }

    /// Flag languages where overhead > threshold or fertility > absolute
    pub fn flag_high_overhead(metrics: &[FertilityMetrics], threshold: OverheadThreshold) -> Vec<FlaggedLanguage> {
        metrics.iter()
            .filter_map(|m| {
                if let Some(overhead) = m.overhead_vs_reference {
                    if overhead > threshold.value() || m.fertility > 2.5 {
                        return Some(FlaggedLanguage {
                            tokenizer: m.tokenizer.clone(),
                            language: m.language.clone(),
                            fertility: m.fertility,
                            overhead,
                        });
                    }
                } else if m.fertility > 3.0 {
                    // no reference, flag absolute
                    return Some(FlaggedLanguage {
                        tokenizer: m.tokenizer.clone(),
                        language: m.language.clone(),
                        fertility: m.fertility,
                        overhead: 0.0,
                    });
                }
                None
            })
            .collect()
    }

    /// Summary statistics per tokenizer
    pub fn summary_stats(metrics: &[FertilityMetrics]) -> HashMap<String, (f64, f64, f64)> {
        // mean, median, variance of fertility per tokenizer
        let mut grouped: HashMap<String, Vec<f64>> = HashMap::new();
        for m in metrics {
            grouped.entry(m.tokenizer.clone()).or_default().push(m.fertility);
        }
        let mut stats = HashMap::new();
        for (tok, mut ferts) in grouped {
            ferts.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mean = ferts.iter().sum::<f64>() / ferts.len() as f64;
            let median = ferts[ferts.len() / 2];
            let var = ferts.iter().map(|f| (f - mean).powi(2)).sum::<f64>() / ferts.len() as f64;
            stats.insert(tok, (mean, median, var));
        }
        stats
    }

    pub fn print_table(metrics: &[FertilityMetrics]) {
        println!("{:<20} {:<8} {:<10} {:<8} {:<8} {:<8} {:<8}", "Tokenizer", "Lang", "Fertility", "CPT", "STRR", "Tokens", "Overhead");
        println!("{}", "-".repeat(80));
        for m in metrics {
            println!("{:<20} {:<8} {:<10.3} {:<8.2} {:<8.3} {:<8} {:<8.2}",
                m.tokenizer, m.language, m.fertility, m.chars_per_token, m.single_token_retention_rate, m.total_tokens,
                m.overhead_vs_reference.unwrap_or(0.0)
            );
        }
    }

    pub fn print_flags(flags: &[FlaggedLanguage], threshold: OverheadThreshold) {
        println!("
=== Flagged High-Overhead (τ = {}) ===", threshold.value());
        for f in flags {
            println!("[{}] {} -> fertility {:.2}, overhead {:.2}x vs ref", f.tokenizer, f.language, f.fertility, f.overhead);
        }
        if flags.is_empty() {
            println!("No language exceeds threshold - tokenizer is multilingual-friendly (Proposition 2 satisfied)");
        } else {
            println!("
Diagnosis: ∀ T ∃ S ⊆ L (Overhead > τ) with S = flagged set - Proposition 3 holds, prevention needed");
        }
    }
}
