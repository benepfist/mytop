use crate::introspection::QueryCacheEntry;
use crate::startup::Config;
use mysql::prelude::Queryable;
use std::collections::BTreeMap;
use std::time::Instant;

pub const SQL_SHOW_FULL_PROCESSLIST: &str = "SHOW FULL PROCESSLIST";
pub const SQL_SHOW_STATUS: &str = "SHOW STATUS";
pub const SQL_SHOW_VARIABLES: &str = "SHOW VARIABLES";
pub const SQL_SHOW_ENGINE_INNODB_STATUS: &str = "SHOW ENGINE INNODB STATUS";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessRow {
    pub id: u64,
    pub user: String,
    pub host: String,
    pub db: Option<String>,
    pub command: String,
    pub time: u64,
    pub state: Option<String>,
    pub info: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataSnapshot {
    pub processlist: Vec<ProcessRow>,
    pub status: BTreeMap<String, String>,
    pub variables: BTreeMap<String, String>,
    pub innodb_status: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PollResult {
    pub snapshot: DataSnapshot,
    pub delta_secs: f64,
    pub status_delta: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, Default)]
pub struct PollState {
    pub qcache: BTreeMap<u64, QueryCacheEntry>,
    pub ucache: BTreeMap<u64, String>,
    pub dbcache: BTreeMap<u64, String>,
    pub statcache: BTreeMap<String, u64>,
    last_poll: Option<Instant>,
}

pub trait DatabaseClient {
    fn show_full_processlist(&mut self) -> Result<Vec<ProcessRow>, String>;
    fn show_status(&mut self) -> Result<BTreeMap<String, String>, String>;
    fn show_variables(&mut self) -> Result<BTreeMap<String, String>, String>;
    fn show_engine_innodb_status(&mut self) -> Result<String, String>;
}

pub struct MysqlClient {
    pool: mysql::Pool,
}

impl MysqlClient {
    pub fn from_config(cfg: &Config) -> Result<Self, String> {
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
        Ok(Self { pool })
    }
}

impl DatabaseClient for MysqlClient {
    fn show_full_processlist(&mut self) -> Result<Vec<ProcessRow>, String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        conn.query_map(
            SQL_SHOW_FULL_PROCESSLIST,
            |(id, user, host, db, command, time, state, info): (
                u64,
                String,
                String,
                Option<String>,
                String,
                u64,
                Option<String>,
                Option<String>,
            )| ProcessRow {
                id,
                user,
                host,
                db,
                command,
                time,
                state,
                info,
            },
        )
        .map_err(|e| e.to_string())
    }

    fn show_status(&mut self) -> Result<BTreeMap<String, String>, String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        let rows: Vec<(String, String)> = conn
            .query(SQL_SHOW_STATUS)
            .map_err(|e| e.to_string())?;
        Ok(rows.into_iter().collect())
    }

    fn show_variables(&mut self) -> Result<BTreeMap<String, String>, String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        let rows: Vec<(String, String)> = conn
            .query(SQL_SHOW_VARIABLES)
            .map_err(|e| e.to_string())?;
        Ok(rows.into_iter().collect())
    }

    fn show_engine_innodb_status(&mut self) -> Result<String, String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        let row: Option<(String, String, String)> = conn
            .query_first(SQL_SHOW_ENGINE_INNODB_STATUS)
            .map_err(|e| e.to_string())?;
        Ok(row.map(|(_, _, status)| status).unwrap_or_default())
    }
}

pub fn poll_once(client: &mut impl DatabaseClient, state: &mut PollState) -> Result<PollResult, String> {
    let now = Instant::now();
    let delta_secs = state
        .last_poll
        .map(|last| (now - last).as_secs_f64().max(0.001))
        .unwrap_or(1.0);

    let processlist = client.show_full_processlist()?;
    let status = client.show_status()?;
    let variables = client.show_variables()?;
    let innodb_status = client.show_engine_innodb_status()?;

    let previous_statcache = state.statcache.clone();
    let current_statcache = status_to_u64_map(&status);
    let status_delta = compute_status_delta(&previous_statcache, &current_statcache);

    update_caches(state, &processlist, &status);

    state.last_poll = Some(now);

    Ok(PollResult {
        snapshot: DataSnapshot {
            processlist,
            status,
            variables,
            innodb_status,
        },
        delta_secs,
        status_delta,
    })
}

fn update_caches(state: &mut PollState, processlist: &[ProcessRow], status: &BTreeMap<String, String>) {
    state.qcache.clear();
    state.ucache.clear();
    state.dbcache.clear();

    for row in processlist {
        if let Some(sql) = row.info.clone() {
            state.qcache.insert(
                row.id,
                QueryCacheEntry {
                    db: row.db.clone(),
                    sql,
                },
            );
        }
        state.ucache.insert(row.id, row.user.clone());
        if let Some(db) = &row.db {
            state.dbcache.insert(row.id, db.clone());
        }
    }

    state.statcache = status_to_u64_map(status);
}

fn status_to_u64_map(status: &BTreeMap<String, String>) -> BTreeMap<String, u64> {
    status
        .iter()
        .filter_map(|(k, v)| v.parse::<u64>().ok().map(|n| (k.clone(), n)))
        .collect()
}

fn compute_status_delta(previous: &BTreeMap<String, u64>, current: &BTreeMap<String, u64>) -> BTreeMap<String, u64> {
    current
        .iter()
        .map(|(k, cur)| {
            let prev = previous.get(k).copied().unwrap_or(0);
            (k.clone(), cur.saturating_sub(prev))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockClient {
        processlist: Vec<ProcessRow>,
        status: BTreeMap<String, String>,
        variables: BTreeMap<String, String>,
        innodb_status: String,
    }

    impl DatabaseClient for MockClient {
        fn show_full_processlist(&mut self) -> Result<Vec<ProcessRow>, String> {
            Ok(self.processlist.clone())
        }

        fn show_status(&mut self) -> Result<BTreeMap<String, String>, String> {
            Ok(self.status.clone())
        }

        fn show_variables(&mut self) -> Result<BTreeMap<String, String>, String> {
            Ok(self.variables.clone())
        }

        fn show_engine_innodb_status(&mut self) -> Result<String, String> {
            Ok(self.innodb_status.clone())
        }
    }

    #[test]
    fn sql_constants_match_expected_queries() {
        assert_eq!(SQL_SHOW_FULL_PROCESSLIST, "SHOW FULL PROCESSLIST");
        assert_eq!(SQL_SHOW_STATUS, "SHOW STATUS");
        assert_eq!(SQL_SHOW_VARIABLES, "SHOW VARIABLES");
        assert_eq!(SQL_SHOW_ENGINE_INNODB_STATUS, "SHOW ENGINE INNODB STATUS");
    }

    #[test]
    fn poll_once_updates_caches_and_snapshot() {
        let mut client = MockClient {
            processlist: vec![ProcessRow {
                id: 7,
                user: "alice".into(),
                host: "db01:43021".into(),
                db: Some("app".into()),
                command: "Query".into(),
                time: 3,
                state: Some("executing".into()),
                info: Some("select 1".into()),
            }],
            status: BTreeMap::from([
                ("Questions".into(), "101".into()),
                ("Threads_connected".into(), "8".into()),
            ]),
            variables: BTreeMap::from([("autocommit".into(), "ON".into())]),
            innodb_status: "ok".into(),
        };

        let mut state = PollState::default();
        let result = poll_once(&mut client, &mut state).unwrap();

        assert_eq!(result.snapshot.processlist.len(), 1);
        assert_eq!(result.snapshot.variables.get("autocommit"), Some(&"ON".to_string()));
        assert_eq!(state.qcache.get(&7).map(|e| e.sql.as_str()), Some("select 1"));
        assert_eq!(state.ucache.get(&7), Some(&"alice".to_string()));
        assert_eq!(state.dbcache.get(&7), Some(&"app".to_string()));
        assert!(result.delta_secs > 0.0);
    }

    #[test]
    fn delta_is_saturating_and_timing_is_robust() {
        let mut client = MockClient {
            processlist: vec![],
            status: BTreeMap::from([("Questions".into(), "50".into())]),
            variables: BTreeMap::new(),
            innodb_status: String::new(),
        };
        let mut state = PollState::default();

        let first = poll_once(&mut client, &mut state).unwrap();
        assert_eq!(first.status_delta.get("Questions"), Some(&0));

        client.status = BTreeMap::from([("Questions".into(), "49".into())]);
        let second = poll_once(&mut client, &mut state).unwrap();

        assert_eq!(second.status_delta.get("Questions"), Some(&0));
        assert!(second.delta_secs >= 0.001);
    }
}
