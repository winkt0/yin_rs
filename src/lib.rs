use std::f64;

#[derive(Clone, Debug)]
pub struct YinResult {
    best_lag: usize,
    cmndf: Vec<f64>,
}

impl YinResult {
    pub fn as_frequency(&self, sample_rate: f64) -> f64 {
        sample_rate / self.best_lag as f64
    }

    pub fn parabolic_interpolation(&self) -> f64 {
        let lag = self.best_lag;
        let cmndf = &self.cmndf;
        let x0 = lag.saturating_sub(1); // max(0, lag-1)
        let x2 = usize::min(cmndf.len() - 1, lag + 1);
        let s0 = cmndf[x0];
        let s1 = cmndf[lag];
        let s2 = cmndf[x2];
        let denom = s0 - 2.0 * s1 + s2;
        if denom == 0.0 {
            return lag as f64;
        }
        let delta = (s0 - s2) / (2.0 * denom);
        lag as f64 + delta
    }
}

#[derive(Clone, Debug)]
pub struct Yin {
    threshold: f64,
    min_lag: usize,
    max_lag: usize,
}

impl Yin {
    pub fn init(threshold: f64, min_lag: usize, max_lag: usize) -> Yin {
        Yin {
            threshold,
            min_lag,
            max_lag,
        }
    }

    pub fn yin(&self, frequencies: &[f64]) -> YinResult {
        let df = df_values(frequencies, self.max_lag);
        let cmndf = cmndf_values(&df, self.max_lag);
        let best_lag = find_cmndf_argmin(&cmndf, self.min_lag, self.max_lag, self.threshold);
        YinResult { best_lag, cmndf }
    }
}

fn df_values(frequencies: &[f64], max_lag: usize) -> Vec<f64> {
    let mut df_list = vec![0.0; frequencies.len()];
    for lag in 1..=max_lag {
        df_list[lag] = df(frequencies, lag);
    }
    df_list
}

fn df(f: &[f64], lag: usize) -> f64 {
    let mut sum = 0.0;
    let n = f.len();
    for i in 0..(n - lag) {
        let diff = f[i] as f64 - f[i + lag] as f64;
        sum += diff * diff;
    }
    sum
}

// Cumulative Mean Normalized Difference Function
fn cmndf_values(df: &[f64], max_lag: usize) -> Vec<f64> {
    let mut cmndf = vec![0.0; max_lag + 1];
    cmndf[0] = 1.0;
    let mut sum = 0.0;
    for lag in 1..=max_lag {
        sum += df[lag];
        cmndf[lag] = lag as f64 * df[lag] / if sum == 0.0 { 1e-10 } else { sum };
    }
    cmndf
}

fn find_cmndf_argmin(cmndf: &[f64], min_lag: usize, max_lag: usize, threshold: f64) -> usize {
    let mut lag = min_lag;
    while lag <= max_lag {
        if cmndf[lag] < threshold {
            while lag + 1 <= max_lag && cmndf[lag + 1] < cmndf[lag] {
                lag += 1;
            }
            return lag;
        }
        lag += 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_sine_wave(frequency: f64, sample_rate: f64, duration_secs: f64) -> Vec<f64> {
        let total_samples = (sample_rate * duration_secs).round() as usize;
        let two_pi_f = 2.0 * std::f64::consts::PI * frequency;

        (0..total_samples)
            .map(|n| {
                let t = n as f64 / sample_rate;
                (two_pi_f * t).sin()
            })
            .collect()
    }

    #[test]
    fn it_works() {
        let sample_rate = 1000.0;
        let frequency = 12.0;
        let seconds = 10.0;
        let signal = generate_sine_wave(frequency, sample_rate, seconds);

        let max_lag = sample_rate / 10.0;
        let min_lag = 1;

        let yin = Yin::init(0.1, min_lag, max_lag.trunc() as usize);
        let result = yin.yin(signal.as_slice());
        let result_freq = result.as_frequency(sample_rate);
        let result_diff_from_actual_freq = (result_freq - frequency).abs();

        assert!(result_diff_from_actual_freq < 1.0);

        let refined_lag = result.parabolic_interpolation();
        let refined_freq = sample_rate / refined_lag;
        let refined_diff = (refined_freq - frequency).abs();
        assert!(refined_diff < 1.0);
        assert!(refined_diff < result_diff_from_actual_freq);
    }
}
