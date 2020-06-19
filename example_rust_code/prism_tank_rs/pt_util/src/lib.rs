use redis;
use std::env;
use log::{debug};


static REDIS_CONN_STR_VAR: &str = "REDIS_CONN_STR";

pub fn get_redis_conn_str() -> Result<String, String> {
    match env::var(REDIS_CONN_STR_VAR) {
        Ok(val) => Ok(val),
        Err(e) => Err(format!(
                "failed to get redis connection string env var {}: {}",
                REDIS_CONN_STR_VAR, e
            ))
    }
}

pub fn init_redis_conn(redis_conn_str: String) -> Result<redis::Connection, String> {

    let redis_client = match redis::Client::open(redis_conn_str) {
        Ok(val) => val,
        Err(e) => return Err(format!("failed to get redis client instance: {}", e)),
    };
    match redis_client.get_connection() {
        Ok(val) => Ok(val),
        Err(e) => Err(format!("failed to get connection: {}", e)),
    }
}

pub fn init_logger() -> Result<(), fern::InitError> {
    // let formatter = syslog::Formatter3164 {
    //     facility: syslog::Facility::LOG_USER,
    //     hostname: None,
    //     process: "prism_tank".to_owned(),
    //     pid: 0,
    // };
    fern::Dispatch::new()
    .chain(
        fern::Dispatch::new()
            .level(log::LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}:{}:{}:{}",
                chrono::Local::now().format("%Y-%m-%d-%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(std::io::stdout())
    )
    // .chain(
    //     fern::Dispatch::new()
    //         .level(log::LevelFilter::Info)
    //         .chain(syslog::unix(syslog::Facility::LOG_USER)?)
    // )    
    .apply()?;

    Ok(())
}

pub fn redis_key_exists(redis_conn: &mut redis::Connection, key: String) -> Result<bool, String> {
    let redis_call_result: redis::RedisResult<bool>;

    redis_call_result = redis::cmd("EXISTS")
        .arg(key)
        .query(redis_conn);
    match redis_call_result {
        Ok(v) => {
            debug!("exists call succeeded: {:?}", v);
            Ok(v)
        },
        Err(e) => {
            Err(format!("exists cmd failed: {}", e))
        }
    }
}

pub fn delete_redis_key(redis_conn: &mut redis::Connection, key: String) -> Result<(), String> {
    let redis_call_result: redis::RedisResult<i32>;
    redis_call_result = redis::cmd("DEL").arg(key).query(redis_conn);
    match redis_call_result {
        Ok(v) => {
            debug!("delete call succeeded: {:?}", v);
            if v > 0 {
                return Ok(());
            }
            Err("no keys deleted".to_string())
        },
        Err(e) => Err(format!("delete call failed: {}", e)),
    }
}

pub fn get_env_var_str(env_var_name: &str) -> Result<String, String> {
    match env::var(env_var_name) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("failed to get env var {}: {:?}", env_var_name, e)),
    }
} 

pub fn get_env_var_i32(env_var_name: &str) -> Result<i32, String> {
    let env_var_str = get_env_var_str(env_var_name)?;
    match env_var_str.parse::<i32>() {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("failed to convert env var {} into i32: {:?}", env_var_name, e)),
    }
}

pub fn get_env_var_u64(env_var_name: &str) -> Result<u64, String> {
    let env_var_str = get_env_var_str(env_var_name)?;
    match env_var_str.parse::<u64>() {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("failed to convert env var {} into u64: {:?}", env_var_name, e)),
    }
}

#[macro_export]
macro_rules! time_secs {
    ($x:expr) => (time::Duration::from_secs($x));
}