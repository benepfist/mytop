use std::path::Path;

pub fn clear_command(is_windows: bool) -> String {
    if is_windows {
        "\n\n\n".to_string()
    } else {
        "clear".to_string()
    }
}

pub fn sum(values: &[u64]) -> u64 {
    values.iter().sum()
}

pub fn commify(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}

pub fn make_short(n: u64) -> String {
    const UNITS: [(&str, u64); 4] = [
        ("T", 1_000_000_000_000),
        ("G", 1_000_000_000),
        ("M", 1_000_000),
        ("k", 1_000),
    ];
    for (unit, div) in UNITS {
        if n >= div {
            return format!("{:.1}{unit}", n as f64 / div as f64);
        }
    }
    n.to_string()
}

pub fn find_prog(name: &str, paths: &[&str]) -> Option<String> {
    for p in paths {
        let candidate = format!("{p}/{name}");
        if Path::new(&candidate).exists() {
            return Some(candidate);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn math_and_number_helpers_work() {
        assert_eq!(sum(&[1, 2, 3]), 6);
        assert_eq!(commify(1_234_567), "1,234,567");
        assert_eq!(make_short(2_200_000), "2.2M");
    }

    #[test]
    fn clear_command_depends_on_platform() {
        assert_eq!(clear_command(false), "clear");
        assert_eq!(clear_command(true), "\n\n\n");
    }

    #[test]
    fn make_short_handles_all_units_and_plain_values() {
        assert_eq!(make_short(999), "999");
        assert_eq!(make_short(1_000), "1.0k");
        assert_eq!(make_short(2_500_000_000), "2.5G");
        assert_eq!(make_short(1_000_000_000_000), "1.0T");
    }

    #[test]
    fn find_prog_returns_first_existing_path() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("mytop-utils-test-{nanos}"));
        let dir_str = dir.to_string_lossy().to_string();
        fs::create_dir_all(&dir).unwrap();

        let bin_name = "mytop-helper";
        let bin_path = dir.join(bin_name);
        fs::write(&bin_path, "echo ok").unwrap();

        let found = find_prog(bin_name, &[&dir_str]).unwrap();
        assert_eq!(found, bin_path.to_string_lossy());
        assert!(find_prog("does-not-exist", &[&dir_str]).is_none());

        fs::remove_dir_all(&dir).unwrap();
    }
}
