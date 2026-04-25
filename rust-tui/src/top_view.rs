#[derive(Debug, Clone, PartialEq)]
pub struct HeaderMetrics {
    pub uptime_secs: u64,
    pub total_qps: f64,
    pub current_qps: f64,
    pub slow_rate: f64,
    pub key_efficiency: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadRow {
    pub id: u64,
    pub user: String,
    pub db: Option<String>,
    pub host: String,
    pub command: String,
    pub time: u64,
}

pub fn compute_header(
    uptime: u64,
    questions: u64,
    delta_questions: u64,
    delta_secs: f64,
    slow: u64,
    key_reads: u64,
    key_read_requests: u64,
) -> HeaderMetrics {
    let total_qps = if uptime == 0 {
        0.0
    } else {
        questions as f64 / uptime as f64
    };
    let current_qps = if delta_secs <= 0.0 {
        0.0
    } else {
        delta_questions as f64 / delta_secs
    };
    let slow_rate = if questions == 0 {
        0.0
    } else {
        slow as f64 / questions as f64
    };
    let key_efficiency = if key_read_requests == 0 {
        1.0
    } else {
        1.0 - (key_reads as f64 / key_read_requests as f64)
    };

    HeaderMetrics {
        uptime_secs: uptime,
        total_qps,
        current_qps,
        slow_rate,
        key_efficiency,
    }
}

pub fn sort_threads_by_time(rows: &mut [ThreadRow], reverse: bool) {
    rows.sort_by_key(|r| r.time);
    if !reverse {
        rows.reverse();
    }
}

pub fn normalize_host(host: &str) -> String {
    host.split(':').next().unwrap_or(host).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_metrics_are_computed() {
        let h = compute_header(100, 1000, 50, 5.0, 10, 5, 100);
        assert_eq!(h.uptime_secs, 100);
        assert_eq!(h.total_qps, 10.0);
        assert_eq!(h.current_qps, 10.0);
        assert!(h.key_efficiency > 0.9);
    }

    #[test]
    fn threads_are_sorted_by_time() {
        let mut rows = vec![
            ThreadRow {
                id: 1,
                user: "a".into(),
                db: None,
                host: "h".into(),
                command: "Sleep".into(),
                time: 1,
            },
            ThreadRow {
                id: 2,
                user: "b".into(),
                db: None,
                host: "h".into(),
                command: "Query".into(),
                time: 8,
            },
        ];

        sort_threads_by_time(&mut rows, false);
        assert_eq!(rows[0].id, 2);
        assert_eq!(normalize_host("db.example.org:3306"), "db.example.org");
    }
}
