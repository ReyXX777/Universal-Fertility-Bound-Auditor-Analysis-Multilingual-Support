use clap::Parser;
use fertility_analyzer::{
    analyzer::FertilityAnalyzer,
    corpus::CorpusCollection,
    report::{ReportGenerator, csv_export},
    tokenizer,
    types::{LanguageCode, OverheadThreshold},
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "fertility_analyzer")]
#[command(about = "Multilingual Fertility & Token Overhead Analysis", long_about = None)]
struct Args {
    #[arg(long, default_value = "en")]
    reference: String,

    #[arg(long, default_value = "2.0")]
    threshold: f64,

    #[arg(long, default_value = "output.csv")]
    csv_out: PathBuf,

    #[arg(long, default_value = "output.json")]
    json_out: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("=== Multilingual Fertility & Token Overhead Analysis ===");
    println!("Reference: {}, Threshold τ = {}", args.reference, args.threshold);

    // Load sample corpora L
    let corpora = CorpusCollection::sample();
    println!("Loaded corpora for languages: {:?}", corpora.languages());

    // Create tokenizers T
    let tokenizers = tokenizer::create_tokenizers();
    println!("Loaded {} tokenizers", tokenizers.len());
    for t in &tokenizers {
        println!(" - {}", t.name());
    }

    // Validate config
    let reference = LanguageCode::new(&args.reference)?;
    let threshold = OverheadThreshold::new(args.threshold)?;

    // Analyze - PROCEDURE step 2
    let analyzer = FertilityAnalyzer::new(&corpora, tokenizers);
    let metrics = analyzer.analyze();
    let metrics_with_overhead = analyzer.compute_overhead(metrics, &reference, threshold);

    // Post-processing - step 3
    let ranked = ReportGenerator::rank_by_fertility(&metrics_with_overhead);
    let flags = ReportGenerator::flag_high_overhead(&metrics_with_overhead, threshold);
    let stats = ReportGenerator::summary_stats(&metrics_with_overhead);

    // Output - step 4
    println!("
--- Detailed Metrics ---");
    ReportGenerator::print_table(&metrics_with_overhead);

    println!("
--- Ranking by Fertility (per tokenizer) ---");
    for (tok, list) in ranked {
        println!("
Tokenizer: {}", tok);
        for (i, m) in list.iter().enumerate() {
            println!("  {}. {} -> {:.3} (overhead {:.2}x)", i+1, m.language, m.fertility, m.overhead_vs_reference.unwrap_or(0.0));
        }
    }

    println!("
--- Summary Stats (mean, median, variance) ---");
    for (tok, (mean, median, var)) in stats {
        println!("{}: mean {:.3}, median {:.3}, var {:.3}", tok, mean, median, var);
    }

    ReportGenerator::print_flags(&flags, threshold);

    // Export
    csv_export::export_csv(&metrics_with_overhead, &args.csv_out)?;
    csv_export::export_json(&metrics_with_overhead, &args.json_out)?;
    println!("
Exported CSV to {:?} and JSON to {:?}", args.csv_out, args.json_out);

    println!("
Goal check: ∃ T* ∀ l Overhead(T*,l) ≤ τ ?");
    if flags.is_empty() {
        println!("Yes - ideal tokenizer found (Proposition 1 approximated)");
    } else {
        println!("Not yet - recommend vocab expansion for flagged languages");
    }

    Ok(())
}
