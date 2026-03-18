use crate::data_models::*;

pub struct CorrelationAnalyzer;

impl CorrelationAnalyzer {
    pub fn analyze_correlation(
        predictions: &[f64],
        measurements: &[f64],
    ) -> CorrelationReport {
        let n = predictions.len().min(measurements.len());
        if n == 0 {
            return CorrelationReport {
                experiment_id: "unknown".to_string(),
                rmse: 0.0,
                pearson_correlation: 0.0,
                threshold_accuracy: 0.0,
                simulation_error_distribution: Vec::new(),
                experimental_variance: 0.0,
            };
        }

        let rmse = Self::calculate_rmse(&predictions[..n], &measurements[..n]);
        let pearson = Self::calculate_pearson(&predictions[..n], &measurements[..n]);
        let errors: Vec<f64> = predictions[..n].iter()
            .zip(measurements[..n].iter())
            .map(|(p, m)| (p - m).abs())
            .collect();
        let experimental_variance = Self::calculate_variance(&measurements[..n]);
        let threshold_accuracy = Self::calculate_threshold_accuracy(&predictions[..n], &measurements[..n], 0.5);

        CorrelationReport {
            experiment_id: format!("CORR_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S")),
            rmse,
            pearson_correlation: pearson,
            threshold_accuracy,
            simulation_error_distribution: errors,
            experimental_variance,
        }
    }

    fn calculate_rmse(predictions: &[f64], measurements: &[f64]) -> f64 {
        let sum_sq: f64 = predictions.iter()
            .zip(measurements.iter())
            .map(|(p, m)| (p - m).powi(2))
            .sum();

        (sum_sq / predictions.len() as f64).sqrt()
    }

    fn calculate_pearson(predictions: &[f64], measurements: &[f64]) -> f64 {
        let n = predictions.len() as f64;

        let mean_p: f64 = predictions.iter().sum::<f64>() / n;
        let mean_m: f64 = measurements.iter().sum::<f64>() / n;

        let numerator: f64 = predictions.iter()
            .zip(measurements.iter())
            .map(|(p, m)| (p - mean_p) * (m - mean_m))
            .sum();

        let sum_p: f64 = predictions.iter().map(|p| (p - mean_p).powi(2)).sum();
        let sum_m: f64 = measurements.iter().map(|m| (m - mean_m).powi(2)).sum();

        let denominator = (sum_p * sum_m).sqrt();

        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }

    fn calculate_variance(data: &[f64]) -> f64 {
        let mean: f64 = data.iter().sum::<f64>() / data.len() as f64;
        let sum_sq: f64 = data.iter().map(|x| (x - mean).powi(2)).sum();
        sum_sq / data.len() as f64
    }

    fn calculate_threshold_accuracy(predictions: &[f64], measurements: &[f64], threshold: f64) -> f64 {
        let correct = predictions.iter()
            .zip(measurements.iter())
            .filter(|(p, m)| (**p > threshold) == (**m > threshold))
            .count();

        correct as f64 / predictions.len() as f64
    }
}
