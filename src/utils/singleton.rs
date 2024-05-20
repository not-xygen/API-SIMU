use std::sync::Arc;

use crate::utils::{
    adapter::connection,
    observer::{LoggerObserver, Observable},
};
use sqlx::mysql::MySqlPool;

pub struct AppState {
    pub db: MySqlPool,
    pub observable: Observable,
}

pub async fn get_app_state() -> Result<Arc<AppState>, sqlx::Error> {
    let observable = Observable::new();
    let logger_observer = Arc::new(LoggerObserver);
    observable.add_observer(logger_observer.clone());

    let pool = connection().await.unwrap();

    let app_state = Arc::new(AppState {
        db: pool,
        observable,
    });

    static mut APP_STATE: Option<Arc<AppState>> = None;
    unsafe {
        if APP_STATE.is_none() {
            APP_STATE = Some(app_state.clone());
        }
        Ok(APP_STATE.as_ref().unwrap().clone())
    }
}
