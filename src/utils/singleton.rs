use std::sync::Arc;
use sqlx::mysql::MySqlPool;
use crate::utils::{
    adapter::connection,
    factory::new_mysql_query_builder,
    observer::{LoggerObserver, Observable},
    factory::QueryBuilder,
};

pub struct AppState {
    pub db: MySqlPool,
    pub observable: Observable,
    pub query_builder: Arc<dyn QueryBuilder>,
}

pub async fn init_app_state() -> Arc<AppState> {
    get_app_state().await.unwrap()
}

pub async fn get_app_state() -> Result<Arc<AppState>, sqlx::Error> {
    let observable = Observable::new();
    let logger_observer = Arc::new(LoggerObserver);
    observable.add_observer(logger_observer.clone());

    let pool = connection().await.unwrap();

    let app_state = Arc::new(AppState {
        db: pool,
        observable,
        query_builder: new_mysql_query_builder(), 
    });
    Ok(app_state)
}
