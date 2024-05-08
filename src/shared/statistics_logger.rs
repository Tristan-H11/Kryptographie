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
}

/// Ein Logger, der keine Statistiken speichert.
pub struct VoidLogger {}

impl StatisticsLogger for VoidLogger {
    fn enrich_context(&mut self, context: &str) {}

    fn remove_context(&mut self) {}

    fn log_statistic(&mut self, key: &str, count: i32) {}

    fn get_all(&self) -> HashMap<String, Vec<i32>> { HashMap::new() }
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