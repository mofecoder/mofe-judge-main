use diesel::{mysql::MysqlConnection, r2d2};
use lazy_init::LazyTransform;
use std::sync::Arc;

#[derive(Clone)]
pub struct MySQLConnPool(
    Arc<LazyTransform<Config, r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>>>,
);

struct Config {
    database_url: String,
    size_conn_pool: u32,
}

fn initialize(config: Config) -> r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>> {
    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(config.database_url);
    r2d2::Pool::builder()
        .max_size(config.size_conn_pool)
        .build(manager)
        .expect("Failed to create pool")
}

impl MySQLConnPool {
    pub fn new(database_url: String, size_conn_pool: u32) -> MySQLConnPool {
        MySQLConnPool(Arc::new(LazyTransform::new(Config {
            database_url,
            size_conn_pool,
        })))
    }

    #[allow(dead_code)]
    pub fn ensure_initialized(&self) {
        self.get_connection();
    }

    #[allow(dead_code)]
    pub fn get_connection(
        &self,
    ) -> r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>> {
        self.0.get_or_create(initialize).get().unwrap()
    }
}
