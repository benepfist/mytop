use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub socket: Option<String>,
    pub user: String,
    pub pass: Option<String>,
    pub db: String,
    pub delay: u64,
    pub nocolor: bool,
    pub batchmode: bool,
    pub prompt_password: bool,
    pub mode: String,
    pub sort: String,
    pub filter_user: String,
    pub filter_db: String,
    pub filter_host: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3306,
            socket: None,
            user: "root".to_string(),
            pass: None,
            db: "test".to_string(),
            delay: 5,
            nocolor: false,
            batchmode: false,
            prompt_password: false,
            mode: "top".to_string(),
            sort: "0".to_string(),
            filter_user: String::new(),
            filter_db: String::new(),
            filter_host: String::new(),
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
            Some((k.trim().to_ascii_lowercase(), v.trim().to_string()))
        })
        .collect()
}

pub fn read_config_file(path: &Path) -> std::io::Result<BTreeMap<String, String>> {
    let content = fs::read_to_string(path)?;
    Ok(parse_kv_config(&content))
}

pub fn parse_cli_args(args: &[String]) -> Result<BTreeMap<String, String>, String> {
    let mut out = BTreeMap::new();
    let mut i = 0usize;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "--help" => {
                out.insert("help".to_string(), "true".to_string());
                i += 1;
            }
            "--color" => {
                out.insert("nocolor".to_string(), "false".to_string());
                i += 1;
            }
            "--nocolor" => {
                out.insert("nocolor".to_string(), "true".to_string());
                i += 1;
            }
            "--batch" | "--batchmode" | "-b" => {
                out.insert("batchmode".to_string(), "true".to_string());
                i += 1;
            }
            "--prompt" => {
                out.insert("prompt".to_string(), "true".to_string());
                i += 1;
            }
            "--header" | "--idle" | "--resolve" | "--long" => {
                i += 1;
            }
            "--user" | "-u" => {
                let value = next_value(args, i, arg)?;
                out.insert("user".to_string(), value);
                i += 2;
            }
            "--pass" | "--password" | "-p" => {
                let value = next_value(args, i, arg)?;
                out.insert("pass".to_string(), value);
                i += 2;
            }
            "--database" | "--db" | "-d" => {
                let value = next_value(args, i, arg)?;
                out.insert("db".to_string(), value);
                i += 2;
            }
            "--host" | "-h" => {
                let value = next_value(args, i, arg)?;
                out.insert("host".to_string(), value);
                i += 2;
            }
            "--port" | "-P" => {
                let value = next_value(args, i, arg)?;
                out.insert("port".to_string(), value);
                i += 2;
            }
            "--socket" | "-S" => {
                let value = next_value(args, i, arg)?;
                out.insert("socket".to_string(), value);
                i += 2;
            }
            "--delay" | "-s" => {
                let value = next_value(args, i, arg)?;
                out.insert("delay".to_string(), value);
                i += 2;
            }
            "--mode" | "-m" => {
                let value = next_value(args, i, arg)?;
                out.insert("mode".to_string(), value);
                i += 2;
            }
            "--sort" => {
                let value = next_value(args, i, arg)?;
                out.insert("sort".to_string(), value);
                i += 2;
            }
            "--filter-user" => {
                let value = next_value(args, i, arg)?;
                out.insert("filter_user".to_string(), value);
                i += 2;
            }
            "--filter-db" => {
                let value = next_value(args, i, arg)?;
                out.insert("filter_db".to_string(), value);
                i += 2;
            }
            "--filter-host" => {
                let value = next_value(args, i, arg)?;
                out.insert("filter_host".to_string(), value);
                i += 2;
            }
            _ if arg.starts_with('-') => {
                return Err(format!("Unknown option: {arg}"));
            }
            _ => {
                i += 1;
            }
        }
    }

    Ok(out)
}

fn next_value(args: &[String], idx: usize, flag: &str) -> Result<String, String> {
    args.get(idx + 1)
        .filter(|v| !v.starts_with('-'))
        .cloned()
        .ok_or_else(|| format!("Missing value for option: {flag}"))
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

    cfg.delay = cfg.delay.max(1);
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
    if let Some(v) = src.get("db") {
        cfg.db = v.clone();
    }
    if let Some(v) = src.get("delay") {
        cfg.delay = v.parse().unwrap_or(cfg.delay);
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
    if let Some(v) = src.get("mode") {
        cfg.mode = v.clone();
    }
    if let Some(v) = src.get("sort") {
        cfg.sort = v.clone();
    }
    if let Some(v) = src.get("filter_user") {
        cfg.filter_user = v.clone();
    }
    if let Some(v) = src.get("filter_db") {
        cfg.filter_db = v.clone();
    }
    if let Some(v) = src.get("filter_host") {
        cfg.filter_host = v.clone();
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
        return format!(
            "DBI:mysql:database={};mysql_read_default_group=mytop;mysql_socket={socket}",
            cfg.db
        );
    }
    format!(
        "DBI:mysql:database={};mysql_read_default_group=mytop;host={};port={}",
        cfg.db, cfg.host, cfg.port
    )
}

pub fn connection_error_message(cfg: &Config, err: &str) -> String {
    format!(
        "Cannot connect to MySQL server. Please check the:\n\n  * database you specified \"{}\" (default is \"test\")\n  * username you specified \"{}\" (default is \"root\")\n  * password you specified \"{}\" (default is \"\")\n  * hostname you specified \"{}\" (default is \"localhost\")\n  * port you specified \"{}\" (default is 3306)\n  * socket you specified \"{}\" (default is \"\")\n\nThe options may be specified on the command-line or in a ~/.mytop\nconfig file. See the manual for details.\n\nHere's the exact error from the Rust MySQL client:\n\n{}\n",
        cfg.db,
        cfg.user,
        cfg.pass.as_deref().unwrap_or(""),
        cfg.host,
        cfg.port,
        cfg.socket.as_deref().unwrap_or(""),
        err
    )
}

pub fn connect_mysql(cfg: &Config) -> Result<(), String> {
    let mut builder = mysql::OptsBuilder::new()
        .user(Some(cfg.user.clone()))
        .pass(cfg.pass.clone())
        .db_name(Some(cfg.db.clone()))
        .tcp_port(cfg.port);

    if let Some(socket) = &cfg.socket {
        builder = builder.socket(Some(socket.clone()));
    } else {
        builder = builder.ip_or_hostname(Some(cfg.host.clone()));
    }

    let pool = mysql::Pool::new(builder).map_err(|e| e.to_string())?;
    let _conn = pool.get_conn().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_default_file_and_cli() {
        let defaults = Config::default();
        let file = parse_kv_config("host=db.internal\nport=3307\nuser=alice\ndb=stage");
        let cli = parse_kv_config("host=db.prod:4406\nuser=bob\nprompt=true\ndelay=2");

        let cfg = merge_config(defaults, file, cli);

        assert_eq!(cfg.host, "db.prod");
        assert_eq!(cfg.port, 4406);
        assert_eq!(cfg.user, "bob");
        assert_eq!(cfg.db, "stage");
        assert_eq!(cfg.delay, 2);
        assert!(cfg.prompt_password);
    }

    #[test]
    fn dsn_prefers_socket() {
        let cfg = Config {
            socket: Some("/tmp/mysql.sock".to_string()),
            ..Config::default()
        };
        assert_eq!(
            build_dsn(&cfg),
            "DBI:mysql:database=test;mysql_read_default_group=mytop;mysql_socket=/tmp/mysql.sock"
        );
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
            db: "app".to_string(),
            ..Config::default()
        };
        assert_eq!(
            build_dsn(&cfg),
            "DBI:mysql:database=app;mysql_read_default_group=mytop;host=db.example;port=3308"
        );
    }

    #[test]
    fn parse_cli_args_maps_short_and_long_options() {
        let args = vec![
            "-u".to_string(),
            "alice".to_string(),
            "--db".to_string(),
            "prod".to_string(),
            "--filter-host".to_string(),
            "db01".to_string(),
            "--batch".to_string(),
        ];

        let parsed = parse_cli_args(&args).unwrap();
        assert_eq!(parsed.get("user"), Some(&"alice".to_string()));
        assert_eq!(parsed.get("db"), Some(&"prod".to_string()));
        assert_eq!(parsed.get("filter_host"), Some(&"db01".to_string()));
        assert_eq!(parsed.get("batchmode"), Some(&"true".to_string()));
    }

    #[test]
    fn parse_cli_args_rejects_unknown_or_missing_values() {
        let args = vec!["--wat".to_string()];
        assert!(parse_cli_args(&args).is_err());

        let args = vec!["--user".to_string()];
        assert!(parse_cli_args(&args).is_err());
    }

    #[test]
    fn connection_error_contains_key_config_values() {
        let cfg = Config {
            host: "db.prod".to_string(),
            port: 3310,
            user: "alice".to_string(),
            pass: Some("secret".to_string()),
            db: "analytics".to_string(),
            socket: Some("/tmp/mysql.sock".to_string()),
            ..Config::default()
        };

        let msg = connection_error_message(&cfg, "access denied");
        assert!(msg.contains("analytics"));
        assert!(msg.contains("alice"));
        assert!(msg.contains("access denied"));
        assert!(msg.contains("/tmp/mysql.sock"));
    }

    #[test]
    fn delay_has_minimum_of_one() {
        let defaults = Config::default();
        let file = parse_kv_config("delay=0");
        let cfg = merge_config(defaults, file, BTreeMap::new());
        assert_eq!(cfg.delay, 1);
    }
}
