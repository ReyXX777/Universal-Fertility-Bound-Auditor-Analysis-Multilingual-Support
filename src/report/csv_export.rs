use crate::metrics::FertilityMetrics;
use crate::error::{AnalyzerError, Result};
use std::path::Path;

pub fn export_csv(metrics: &[FertilityMetrics], path: &Path) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path).map_err(|e| AnalyzerError::Io(e.to_string()))?;
    for m in metrics {
        wtr.serialize(m).map_err(|e| AnalyzerError::Io(e.to_string()))?;
    }
    wtr.flush().map_err(|e| AnalyzerError::Io(e.to_string()))?;
    Ok(())
}

pub fn export_json(metrics: &[FertilityMetrics], path: &Path) -> Result<()> {
    let file = std::fs::File::create(path).map_err(|e| AnalyzerError::Io(e.to_string()))?;
    serde_json::to_writer_pretty(file, metrics).map_err(|e| AnalyzerError::Io(e.to_string()))?;
    Ok(())
}
