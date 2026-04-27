use std::collections::BTreeMap;

pub fn get_qps(previous: u64, current: u64) -> u64 {
    current.saturating_sub(previous)
}

pub fn get_qps_rate(previous: u64, current: u64, delta_secs: f64) -> f64 {
    if delta_secs <= 0.0 {
        0.0
    } else {
        get_qps(previous, current) as f64 / delta_secs
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CmdSummary {
    pub name: String,
    pub total: u64,
    pub pct: f64,
    pub delta: u64,
    pub delta_pct: f64,
}

pub fn command_summary(
    previous: &BTreeMap<String, u64>,
    current: &BTreeMap<String, u64>,
) -> Vec<CmdSummary> {
    let total: u64 = current.values().sum();
    current
        .iter()
        .filter(|(k, _)| k.starts_with("Com_"))
        .map(|(k, cur)| {
            let prev = previous.get(k).copied().unwrap_or(0);
            let delta = cur.saturating_sub(prev);
            let name = k.trim_start_matches("Com_").replace('_', " ");
            let pct = if total == 0 {
                0.0
            } else {
                *cur as f64 / total as f64
            };
            let delta_pct = if *cur == 0 {
                0.0
            } else {
                delta as f64 / *cur as f64
            };
            CmdSummary {
                name,
                total: *cur,
                pct,
                delta,
                delta_pct,
            }
        })
        .collect()
}

pub fn show_status(
    previous: &BTreeMap<String, u64>,
    current: &BTreeMap<String, u64>,
) -> Vec<(String, u64, u64)> {
    current
        .iter()
        .filter(|(k, _)| !k.starts_with("Com_"))
        .map(|(k, cur)| {
            (
                k.clone(),
                *cur,
                cur.saturating_sub(previous.get(k).copied().unwrap_or(0)),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qps_is_delta_of_questions() {
        assert_eq!(get_qps(100, 130), 30);
        assert_eq!(get_qps_rate(100, 130, 3.0), 10.0);
        assert_eq!(get_qps_rate(100, 130, 0.0), 0.0);
    }

    #[test]
    fn command_summary_normalizes_names() {
        let mut prev = BTreeMap::new();
        prev.insert("Com_select".into(), 10);
        let mut cur = BTreeMap::new();
        cur.insert("Com_select".into(), 20);
        cur.insert("Threads_connected".into(), 5);

        let s = command_summary(&prev, &cur);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].name, "select");
        assert_eq!(s[0].delta, 10);
    }

    #[test]
    fn show_status_ignores_com_values() {
        let prev = BTreeMap::new();
        let mut cur = BTreeMap::new();
        cur.insert("Com_insert".into(), 2);
        cur.insert("Threads_running".into(), 3);

        let rows = show_status(&prev, &cur);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, "Threads_running");
    }
}
