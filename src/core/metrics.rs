use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Metrics collector for performance monitoring
pub struct MetricsCollector {
    metrics: Arc<Mutex<Metrics>>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(Metrics::new())),
        }
    }

    /// Record a metric
    pub fn record(&self, name: &str, value: f64) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.record(name, value);
        }
    }

    /// Increment a counter
    pub fn increment(&self, name: &str) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.increment(name);
        }
    }

    /// Record timing
    pub fn record_timing(&self, name: &str, duration: Duration) {
        self.record(name, duration.as_secs_f64());
    }

    /// Get all metrics
    pub fn get_metrics(&self) -> HashMap<String, MetricValue> {
        if let Ok(metrics) = self.metrics.lock() {
            metrics.get_all()
        } else {
            HashMap::new()
        }
    }

    /// Reset all metrics
    pub fn reset(&self) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.reset();
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal metrics storage
struct Metrics {
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
    timings: HashMap<String, Vec<f64>>,
    start_time: Instant,
}

impl Metrics {
    fn new() -> Self {
        Self {
            counters: HashMap::new(),
            gauges: HashMap::new(),
            timings: HashMap::new(),
            start_time: Instant::now(),
        }
    }

    fn record(&mut self, name: &str, value: f64) {
        self.gauges.insert(name.to_string(), value);
    }

    fn increment(&mut self, name: &str) {
        *self.counters.entry(name.to_string()).or_insert(0) += 1;
    }

    fn get_all(&self) -> HashMap<String, MetricValue> {
        let mut result = HashMap::new();

        // Add counters
        for (name, value) in &self.counters {
            result.insert(name.clone(), MetricValue::Counter(*value));
        }

        // Add gauges
        for (name, value) in &self.gauges {
            result.insert(name.clone(), MetricValue::Gauge(*value));
        }

        // Add timing statistics
        for (name, values) in &self.timings {
            if !values.is_empty() {
                let sum: f64 = values.iter().sum();
                let avg = sum / values.len() as f64;
                let min = values.iter().copied().fold(f64::INFINITY, f64::min);
                let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                result.insert(name.clone(), MetricValue::Timing {
                    count: values.len(),
                    avg,
                    min,
                    max,
                });
            }
        }

        // Add uptime
        let uptime = self.start_time.elapsed().as_secs_f64();
        result.insert("uptime".to_string(), MetricValue::Gauge(uptime));

        result
    }

    fn reset(&mut self) {
        self.counters.clear();
        self.gauges.clear();
        self.timings.clear();
        self.start_time = Instant::now();
    }
}

/// Metric value types
#[derive(Debug, Clone)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Timing {
        count: usize,
        avg: f64,
        min: f64,
        max: f64,
    },
}

