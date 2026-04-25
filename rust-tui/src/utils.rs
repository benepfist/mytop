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

    #[test]
    fn math_and_number_helpers_work() {
        assert_eq!(sum(&[1, 2, 3]), 6);
        assert_eq!(commify(1_234_567), "1,234,567");
        assert_eq!(make_short(2_200_000), "2.2M");
    }

    #[test]
    fn clear_command_depends_on_platform() {
        assert_eq!(clear_command(false), "clear");
    }
}
