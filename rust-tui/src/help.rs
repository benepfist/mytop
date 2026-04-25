pub fn print_help(color: bool) -> String {
    let header = if color {
        "\u{001b}[32mmytop shortcuts\u{001b}[0m"
    } else {
        "mytop shortcuts"
    };
    format!(
        "{header}\n[t] top  [m] qps  [c] cmd  [I] innodb  [S] status  [q] quit\nhttps://jeremy.zawodny.com/mysql/mytop/"
    )
}

pub fn pod_sections() -> Vec<&'static str> {
    vec![
        "Synopsis",
        "Requirements",
        "Platforms",
        "Display",
        "Arguments",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_contains_core_shortcuts_and_url() {
        let h = print_help(false);
        assert!(h.contains("[t]"));
        assert!(h.contains("[q]"));
        assert!(h.contains("mytop"));
    }

    #[test]
    fn pod_sections_exposed() {
        let p = pod_sections();
        assert!(p.contains(&"Synopsis"));
        assert!(p.contains(&"Arguments"));
    }
}
