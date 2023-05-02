use anyhow::{anyhow, Result};
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Pool as PGPool, Runtime};

/// Config URL is in format:
/// postgres://USER:PASSWORD@HOST:PORT/DBNAME
fn pg_config_from_url(url: &str) -> Result<Config> {
    let url = url::Url::parse(url).map_err(|_| anyhow!("could not parse db url"))?;
    let user = url.username().to_string();
    let password = url.password().ok_or(anyhow!("bad password"))?.to_string();
    let host = url.host().ok_or(anyhow!("bad host"))?.to_string();
    // let port = url.port().ok_or("no port in url")?;
    let dbname = url
        .path_segments()
        .ok_or(anyhow!("cannot be base"))?
        .next()
        .unwrap()
        .to_string();

    let mut cfg = Config::new();
    cfg.user = Some(user);
    cfg.password = Some(password);
    cfg.host = Some(host);
    // cfg.port = Some(port);
    cfg.dbname = Some(dbname);

    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    Ok(cfg)
}

pub fn pg_pool_from_url(url: &str) -> Result<PGPool> {
    let pg_cfg = pg_config_from_url(url)?;

    // TODO: Add better error context as one possible error is just 'Bad password'
    Ok(pg_cfg.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)?)
}
