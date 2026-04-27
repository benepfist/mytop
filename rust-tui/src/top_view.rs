use crate::filters::Filters;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TopViewOptions {
    pub sort_desc: bool,
    pub hide_idle: bool,
}

impl Default for TopViewOptions {
    fn default() -> Self {
        Self {
            sort_desc: true,
            hide_idle: false,
        }
    }
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

pub fn prepare_top_rows(
    rows: &[ThreadRow],
    filters: &Filters,
    opts: TopViewOptions,
) -> Vec<ThreadRow> {
    let mut filtered = rows
        .iter()
        .filter(|row| {
            filters.user.matches(&row.user)
                && filters.host.matches(&normalize_host(&row.host))
                && filters.db.matches(row.db.as_deref().unwrap_or(""))
                && (!opts.hide_idle || row.command != "Sleep")
        })
        .cloned()
        .collect::<Vec<_>>();

    sort_threads_by_time(&mut filtered, opts.sort_desc);
    filtered
}

pub fn sort_threads_by_time(rows: &mut [ThreadRow], desc: bool) {
    rows.sort_by_key(|r| r.time);
    if desc {
        rows.reverse();
    }
}

pub fn normalize_host(host: &str) -> String {
    host.split(':').next().unwrap_or(host).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{Filters, StringOrRegex};

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

        sort_threads_by_time(&mut rows, true);
        assert_eq!(rows[0].id, 2);
        assert_eq!(normalize_host("db.example.org:3306"), "db.example.org");
    }

    #[test]
    fn prepare_top_rows_applies_filters_idle_and_sorting() {
        let rows = vec![
            ThreadRow {
                id: 10,
                user: "alice".into(),
                db: Some("app".into()),
                host: "db01:42000".into(),
                command: "Sleep".into(),
                time: 20,
            },
            ThreadRow {
                id: 11,
                user: "alice".into(),
                db: Some("app".into()),
                host: "db01:43000".into(),
                command: "Query".into(),
                time: 5,
            },
        ];
        let filters = Filters {
            user: StringOrRegex::Exact("alice".into()),
            db: StringOrRegex::Exact("app".into()),
            host: StringOrRegex::Exact("db01".into()),
        };

        let out = prepare_top_rows(
            &rows,
            &filters,
            TopViewOptions {
                sort_desc: true,
                hide_idle: true,
            },
        );

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, 11);
    }
}
