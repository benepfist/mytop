use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub socket: Option<String>,
    pub user: String,
    pub pass: Option<String>,
    pub nocolor: bool,
    pub batchmode: bool,
    pub prompt_password: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3306,
            socket: None,
            user: "root".to_string(),
            pass: None,
            nocolor: false,
            batchmode: false,
            prompt_password: false,
        }
    }
}

pub fn parse_kv_config(content: &str) -> BTreeMap<String, String> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                return None;
            }
            let (k, v) = trimmed.split_once('=')?;
            Some((k.trim().to_string(), v.trim().to_string()))
        })
        .collect()
}

pub fn merge_config(
    defaults: Config,
    file: BTreeMap<String, String>,
    cli: BTreeMap<String, String>,
) -> Config {
    let mut cfg = defaults;

    apply_map(&mut cfg, &file);
    apply_map(&mut cfg, &cli);

    if let Some((h, p)) = cfg.host.clone().split_once(':') {
        cfg.host = h.to_string();
        cfg.port = p.parse().unwrap_or(cfg.port);
    }

    cfg
}

fn apply_map(cfg: &mut Config, src: &BTreeMap<String, String>) {
    if let Some(v) = src.get("host") {
        cfg.host = v.clone();
    }
    if let Some(v) = src.get("port") {
        cfg.port = v.parse().unwrap_or(cfg.port);
    }
    if let Some(v) = src.get("socket") {
        cfg.socket = Some(v.clone());
    }
    if let Some(v) = src.get("user") {
        cfg.user = v.clone();
    }
    if let Some(v) = src.get("pass") {
        cfg.pass = Some(v.clone());
    }
    if let Some(v) = src.get("nocolor") {
        cfg.nocolor = v == "1" || v.eq_ignore_ascii_case("true");
    }
    if let Some(v) = src.get("batchmode") {
        cfg.batchmode = v == "1" || v.eq_ignore_ascii_case("true");
    }
    if let Some(v) = src.get("prompt") {
        cfg.prompt_password = v == "1" || v.eq_ignore_ascii_case("true");
    }
}

pub fn use_interactive_keyboard(cfg: &Config) -> bool {
    !cfg.batchmode
}

pub fn color_enabled(cfg: &Config, is_windows: bool) -> bool {
    !cfg.nocolor && !is_windows
}

pub fn build_dsn(cfg: &Config) -> String {
    if let Some(socket) = &cfg.socket {
        return format!("mysql:socket={socket};user={}", cfg.user);
    }
    format!(
        "mysql:host={};port={};user={}",
        cfg.host, cfg.port, cfg.user
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_default_file_and_cli() {
        let defaults = Config::default();
        let file = parse_kv_config("host=db.internal\nport=3307\nuser=alice");
        let cli = parse_kv_config("host=db.prod:4406\nuser=bob\nprompt=true");

        let cfg = merge_config(defaults, file, cli);

        assert_eq!(cfg.host, "db.prod");
        assert_eq!(cfg.port, 4406);
        assert_eq!(cfg.user, "bob");
        assert!(cfg.prompt_password);
    }

    #[test]
    fn dsn_prefers_socket() {
        let cfg = Config {
            socket: Some("/tmp/mysql.sock".to_string()),
            ..Config::default()
        };
        assert_eq!(build_dsn(&cfg), "mysql:socket=/tmp/mysql.sock;user=root");
    }

    #[test]
    fn interactive_and_color_flags_work() {
        let cfg = Config::default();
        assert!(use_interactive_keyboard(&cfg));
        assert!(color_enabled(&cfg, false));
        assert!(!color_enabled(&cfg, true));
    }

    #[test]
    fn parse_kv_config_skips_comments_and_invalid_lines() {
        let parsed = parse_kv_config(
            "
                # comment
                host = db.local
                invalid-line
                nocolor=true
            ",
        );

        assert_eq!(parsed.get("host"), Some(&"db.local".to_string()));
        assert_eq!(parsed.get("nocolor"), Some(&"true".to_string()));
        assert!(!parsed.contains_key("invalid-line"));
    }

    #[test]
    fn merge_config_parses_boolean_and_handles_invalid_port() {
        let defaults = Config::default();
        let file = parse_kv_config(
            "port=not-a-number\nbatchmode=1\nnocolor=false\nsocket=/var/lib/mysql.sock\npass=s3cr3t",
        );
        let cli = parse_kv_config("nocolor=true");

        let cfg = merge_config(defaults, file, cli);
        assert_eq!(cfg.port, 3306);
        assert!(cfg.batchmode);
        assert!(cfg.nocolor);
        assert_eq!(cfg.socket, Some("/var/lib/mysql.sock".to_string()));
        assert_eq!(cfg.pass, Some("s3cr3t".to_string()));
        assert!(!use_interactive_keyboard(&cfg));
    }

    #[test]
    fn build_dsn_uses_host_and_port_when_socket_missing() {
        let cfg = Config {
            host: "db.example".to_string(),
            port: 3308,
            user: "alice".to_string(),
            ..Config::default()
        };
        assert_eq!(
            build_dsn(&cfg),
            "mysql:host=db.example;port=3308;user=alice"
        );
    }
}
