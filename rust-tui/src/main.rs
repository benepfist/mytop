use mytop_tui::startup::{
    Config, color_enabled, connect_mysql, connection_error_message, merge_config, parse_cli_args,
    read_config_file,
};
use mytop_tui::{help, loop_modes};
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    let raw_args: Vec<String> = std::env::args().skip(1).collect();
    let cli_map = match parse_cli_args(&raw_args) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e}");
            print_usage();
            std::process::exit(2);
        }
    };

    if cli_map.contains_key("help") {
        print_usage();
        println!();
        println!("{}", help::print_help(color_enabled(&Config::default(), cfg!(windows))));
        let sections = help::pod_sections().join(", ");
        println!("POD: {sections}");
        return;
    }

    let file_map = load_user_config();
    let mut cfg = merge_config(Config::default(), file_map, cli_map);

    if cfg.prompt_password && cfg.pass.is_none() {
        cfg.pass = Some(prompt_password());
    }

    let _color = color_enabled(&cfg, cfg!(windows));

    match connect_mysql(&cfg) {
        Ok(()) => {
            let startup_mode = loop_modes::parse_key(match cfg.mode.as_str() {
                "qps" => 'm',
                "cmd" => 'c',
                "innodb" => 'I',
                "status" => 'S',
                _ => 't',
            });
            println!(
                "mytop-tui startup ready (host={}, port={}, db={}, mode={}, key={:?})",
                cfg.host, cfg.port, cfg.db, cfg.mode, startup_mode
            );
        }
        Err(err) => {
            eprintln!("{}", connection_error_message(&cfg, &err));
            std::process::exit(1);
        }
    }
}

fn load_user_config() -> BTreeMap<String, String> {
    let Some(home) = std::env::var_os("HOME") else {
        return BTreeMap::new();
    };

    let mut config_path = PathBuf::from(home);
    config_path.push(".mytop");

    if !config_path.exists() {
        return BTreeMap::new();
    }

    match read_config_file(&config_path) {
        Ok(map) => map,
        Err(err) => {
            eprintln!(
                "Warning: could not read config file {}: {err}",
                config_path.display()
            );
            BTreeMap::new()
        }
    }
}

fn prompt_password() -> String {
    print!("Password: ");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
    buf.trim_end().to_string()
}

fn print_usage() {
    println!(
        "Usage: mytop-tui [options]\n\
         Options:\n\
           -u, --user <USER>             Database user\n\
           -p, --pass <PASS>             Password\n\
           -d, --db, --database <DB>     Database name\n\
           -h, --host <HOST>             Host (host:port supported)\n\
           -P, --port <PORT>             TCP port\n\
           -S, --socket <SOCKET>         Unix socket path\n\
           -s, --delay <SECS>            Refresh delay\n\
           -b, --batch                    Batch mode\n\
               --prompt                   Prompt for password if missing\n\
               --color | --nocolor        Toggle color output\n\
           -m, --mode <MODE>             Startup mode (top/qps/cmd/innodb/status)\n\
               --sort <SORT>              Sort mode\n\
               --filter-user <PATTERN>    User filter\n\
               --filter-db <PATTERN>      DB filter\n\
               --filter-host <PATTERN>    Host filter\n\
               --help                     Show this help"
    );
}
