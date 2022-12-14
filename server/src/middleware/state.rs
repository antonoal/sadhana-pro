use crate::utils;
use common::error::AppError;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};

type AppConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: utils::db::DbPool,
}

impl AppState {
    pub fn get_conn(&self) -> Result<AppConn, AppError> {
        let conn = self.pool.get()?;
        Ok(conn)
    }

    pub fn init() -> Self {
        let pool = utils::db::establish_connection();
        AppState { pool }
    }
}
