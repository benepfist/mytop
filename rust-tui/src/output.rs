use crate::summaries::CmdSummary;
use crate::top_view::{HeaderMetrics, ThreadRow, normalize_host};
use std::collections::BTreeMap;

pub fn format_error(message: &str) -> String {
    let trimmed = message.trim();
    if trimmed.starts_with("***") && trimmed.ends_with("***") {
        trimmed.to_string()
    } else {
        format!("*** {trimmed} ***")
    }
}

pub fn format_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let mut out = vec![headers.join(" | ")];
    out.extend(rows.iter().map(|r| r.join(" | ")));
    out.join("\n")
}

pub fn format_show_variables(rows: &BTreeMap<String, String>) -> String {
    let mut table_rows = rows
        .iter()
        .map(|(k, v)| vec![k.clone(), v.clone()])
        .collect::<Vec<_>>();
    table_rows.sort_by(|a, b| a[0].cmp(&b[0]));
    format_table(&["Variable", "Value"], &table_rows)
}

pub fn render_top_header(metrics: &HeaderMetrics, threads: usize, color: bool) -> String {
    let base = format!(
        "Uptime: {}s  Threads: {}  Questions: {:.2} qps(avg) / {:.2} qps(now)  Slow: {:.2}%  KeyEff: {:.2}%",
        metrics.uptime_secs,
        threads,
        metrics.total_qps,
        metrics.current_qps,
        metrics.slow_rate * 100.0,
        metrics.key_efficiency * 100.0
    );
    if color {
        format!("\u{001b}[32m{base}\u{001b}[0m")
    } else {
        base
    }
}

pub fn render_top_table(rows: &[ThreadRow], color: bool) -> String {
    let row_data = rows
        .iter()
        .map(|r| {
            let mut row = vec![
                r.id.to_string(),
                r.user.clone(),
                r.db.clone().unwrap_or_else(|| "-".to_string()),
                normalize_host(&r.host),
                r.command.clone(),
                r.time.to_string(),
            ];
            if color && r.command == "Query" {
                row[4] = format!("\u{001b}[33m{}\u{001b}[0m", row[4]);
            }
            row
        })
        .collect::<Vec<_>>();

    format_table(&["Id", "User", "DB", "Host", "Command", "Time"], &row_data)
}

pub fn render_qps_view(previous_questions: u64, current_questions: u64, delta_secs: f64) -> String {
    let delta = current_questions.saturating_sub(previous_questions);
    let qps = if delta_secs <= 0.0 {
        0.0
    } else {
        delta as f64 / delta_secs
    };
    format!(
        "QPS View\nQuestions previous={previous_questions} current={current_questions} delta={delta}\nqps={qps:.2}"
    )
}

pub fn render_cmd_view(rows: &[CmdSummary]) -> String {
    let row_data = rows
        .iter()
        .map(|r| {
            vec![
                r.name.clone(),
                r.total.to_string(),
                format!("{:.2}%", r.pct * 100.0),
                r.delta.to_string(),
                format!("{:.2}%", r.delta_pct * 100.0),
            ]
        })
        .collect::<Vec<_>>();

    format!(
        "Command Summary\n{}",
        format_table(&["Name", "Total", "Pct", "Delta", "DeltaPct"], &row_data)
    )
}

pub fn render_status_view(rows: &[(String, u64, u64)]) -> String {
    let row_data = rows
        .iter()
        .map(|(name, total, delta)| vec![name.clone(), total.to_string(), delta.to_string()])
        .collect::<Vec<_>>();

    format!(
        "Status Summary\n{}",
        format_table(&["Name", "Total", "Delta"], &row_data)
    )
}

pub fn innodb_status_text(status: &str) -> String {
    status.to_string()
}

pub fn find_pager(available: &[&str]) -> String {
    if available.contains(&"less") {
        "less".to_string()
    } else {
        "more".to_string()
    }
}

pub fn render_innodb_view(status: &str, available_pagers: &[&str]) -> String {
    let pager = find_pager(available_pagers);
    format!("InnoDB Status (pager={pager})\n{}", innodb_status_text(status))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pager_prefers_less_with_more_fallback() {
        assert_eq!(find_pager(&["cat", "less"]), "less");
        assert_eq!(find_pager(&["cat"]), "more");
    }

    #[test]
    fn variables_are_formatted_as_table_and_error_format_is_unified() {
        let mut v = BTreeMap::new();
        v.insert("autocommit".into(), "ON".into());
        let text = format_show_variables(&v);
        assert!(text.contains("Variable | Value"));
        assert!(text.contains("autocommit | ON"));

        assert_eq!(format_error("Invalid id."), "*** Invalid id. ***");
        assert_eq!(format_error("*** Invalid id. ***"), "*** Invalid id. ***");
    }

    #[test]
    fn header_and_top_table_rendering_support_color_and_host_normalization() {
        let header = render_top_header(
            &HeaderMetrics {
                uptime_secs: 120,
                total_qps: 10.0,
                current_qps: 5.0,
                slow_rate: 0.1,
                key_efficiency: 0.95,
            },
            3,
            true,
        );
        assert!(header.contains("\u{001b}[32m"));

        let table = render_top_table(
            &[ThreadRow {
                id: 1,
                user: "alice".into(),
                db: Some("app".into()),
                host: "db01:3306".into(),
                command: "Query".into(),
                time: 2,
            }],
            true,
        );
        assert!(table.contains("db01"));
        assert!(table.contains("\u{001b}[33mQuery\u{001b}[0m"));
    }

    #[test]
    fn qps_cmd_status_innodb_views_are_rendered() {
        let qps = render_qps_view(100, 130, 5.0);
        assert!(qps.contains("qps=6.00"));

        let cmd = render_cmd_view(&[CmdSummary {
            name: "select".into(),
            total: 20,
            pct: 0.5,
            delta: 4,
            delta_pct: 0.2,
        }]);
        assert!(cmd.contains("Name | Total | Pct | Delta | DeltaPct"));
        assert!(cmd.contains("select | 20 | 50.00% | 4 | 20.00%"));

        let status = render_status_view(&[("Threads_running".into(), 10, 1)]);
        assert!(status.contains("Name | Total | Delta"));
        assert!(status.contains("Threads_running | 10 | 1"));

        let innodb = render_innodb_view("LATEST DETECTED DEADLOCK", &["cat", "more"]);
        assert!(innodb.contains("pager=more"));
        assert!(innodb.contains("LATEST DETECTED DEADLOCK"));
    }
}
