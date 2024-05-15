use std::collections::HashMap;

pub trait StatisticsLogger {
    /// Fügt eine Kontextschicht hinzu.
    fn enrich_context(&mut self, context: &str);

    /// Entfernt die oberste Kontextschicht.
    fn remove_context(&mut self);

    /// Speichert eine Statistik.
    fn log_statistic(&mut self, key: &str, count: i32);

    /// Gibt alle gespeicherten Statistiken zurück.
    fn get_all(&self) -> HashMap<String, Vec<i32>>;

    /// Gibt die Statistiken als Durchschnitt und Standardabweichung zurück.
    fn get_all_with_metrics(&self) -> HashMap<String, Vec<(String, f64)>>;
}

pub struct StatisticsLoggerImpl {
    contexts: Vec<String>,
    data: HashMap<String, Vec<i32>>,
}

impl StatisticsLoggerImpl {
    pub fn new() -> StatisticsLoggerImpl {
        StatisticsLoggerImpl {
            contexts: Vec::new(),
            data: HashMap::new(),
        }
    }

    fn mean(&self, data: &[i32]) -> Option<f32> {
        let sum = data.iter().sum::<i32>() as f32;
        let count = data.len();

        match count {
            positive if positive > 0 => Some(sum / count as f32),
            _ => None,
        }
    }

    fn std_deviation(&self, data: &[i32]) -> Option<f32> {
        match (self.mean(data), data.len()) {
            (Some(data_mean), count) if count > 0 => {
                let variance = data.iter().map(|value| {
                    let diff = data_mean - (*value as f32);
                    diff * diff
                }).sum::<f32>() / count as f32;

                Some(variance.sqrt())
            },
            _ => None
        }
    }

    fn first_and_third_quartile(&self, data: &[i32]) -> Option<(i32, i32)> {
        let mut sorted_data = data.to_vec();
        sorted_data.sort();

        let count = sorted_data.len();
        match count {
            positive if positive > 0 => {
                let first_index = count / 4;
                let third_index = count * 3 / 4;
                Some((sorted_data[first_index], sorted_data[third_index]))
            },
            _ => None
        }
    }

    fn min_and_max(&self, data: &[i32]) -> Option<(i32, i32)> {
        let mut sorted_data = data.to_vec();
        sorted_data.sort();

        let count = sorted_data.len();
        match count {
            positive if positive > 0 => Some((sorted_data[0], sorted_data[count - 1])),
            _ => None
        }
    }
}

impl StatisticsLogger for StatisticsLoggerImpl {
    fn enrich_context(&mut self, context: &str) {
        self.contexts.push(context.to_string());
    }

    fn remove_context(&mut self) {
        self.contexts.pop();
    }

    fn log_statistic(&mut self, key: &str, count: i32) {
        let mut full_key = String::new();
        for context in &self.contexts {
            full_key.push_str(context);
            full_key.push_str(" :: ");
        }
        full_key.push_str(key);

        let values = self.data.entry(full_key.clone()).or_insert(Vec::new());
        values.push(count);
    }

    fn get_all(&self) -> HashMap<String, Vec<i32>> {
        self.data.clone()
    }

    fn get_all_with_metrics(&self) -> HashMap<String, Vec<(String, f64)>> {
        let mut result = HashMap::new();
        for (key, values) in &self.data {

            // Abschneiden der oberen und unteren 10%
            let mut values = values.clone();
            values.sort();
            let ten_percent = (values.len() as f64 * 0.1) as usize;
            values = values[ten_percent..values.len() - ten_percent].to_vec();

            let avg = self.mean(&values).unwrap_or(0.0) as f64;
            let stddev = self.std_deviation(&values).unwrap_or(0.0) as f64;
            let first_and_third = self.first_and_third_quartile(&values).unwrap_or((0, 0));
            let min_and_max = self.min_and_max(&values).unwrap_or((0, 0));

            let vec = vec![
                ("Average: ".into(), avg),
                ("StdDev: ".into(), stddev),
                ("First Quartile: ".into(), first_and_third.0 as f64),
                ("Third Quartile: ".into(), first_and_third.1 as f64),
                ("Min: ".into(), min_and_max.0 as f64),
                ("Max: ".into(), min_and_max.1 as f64)
            ];

            result.insert(
                key.clone(),
                vec
            );
        }
        result
    }


}

/// Ein Logger, der keine Statistiken speichert.
pub struct VoidLogger {}

impl StatisticsLogger for VoidLogger {
    fn enrich_context(&mut self, _context: &str) {}

    fn remove_context(&mut self) {}

    fn log_statistic(&mut self, _key: &str, _count: i32) {}

    fn get_all(&self) -> HashMap<String, Vec<i32>> { HashMap::new() }

    fn get_all_with_metrics(&self) -> HashMap<String, Vec<(String, f64)>> {
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_logger() {
        let mut logger = StatisticsLoggerImpl::new();
        logger.enrich_context("context1");
        logger.enrich_context("context2");
        logger.log_statistic("stat1", 1);
        logger.log_statistic("stat1", 2);
        logger.log_statistic("stat2", 3);
        logger.remove_context();
        logger.log_statistic("stat1", 4);

        let data = logger.get_all();
        assert_eq!(data.len(), 4);
        assert_eq!(data.get("context1-context2-stat1").unwrap(), &vec![1, 2]);
        assert_eq!(data.get("context1-context2-stat2").unwrap(), &vec![3]);
        assert_eq!(data.get("context1-stat1").unwrap(), &vec![4]);
    }
}