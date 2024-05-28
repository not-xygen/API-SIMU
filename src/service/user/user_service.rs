use crate::{
    model::{model::UserModel, schema::CreateUpdateUserSchema},
    service::user::user_validator::{create_validation, update_validation},
    utils::singleton::{init_app_state, AppState},
};
use axum::{extract::State, Json};
use std::sync::Arc;
use tokio::sync::OnceCell;

static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::const_new();

pub async fn get_all_user_service() -> Result<Vec<UserModel>, String> {
    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();
    let data: State<Arc<AppState>> = State(app_state);
    let query = data.query_builder.select_all("users");
    let res = sqlx::query_as::<_, UserModel>(&query)
        .fetch_all(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    res
}

pub async fn get_user_by_id_service(id: i32) -> Result<UserModel, String> {
    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();
    let data: State<Arc<AppState>> = State(app_state);
    let query = data.query_builder.select_by_id("users", id as u64);
    let res = sqlx::query_as::<_, UserModel>(&query)
        .fetch_one(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    if let Err(_) = res {
        return Err(format!("User with id {} does not exist", id));
    }

    res
}

pub async fn create_user_service(
    Json(body): Json<CreateUpdateUserSchema>,
) -> Result<sqlx::mysql::MySqlQueryResult, String> {
    create_validation(&body).await?;
    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();
    let data: State<Arc<AppState>> = State(app_state);
    let password = bcrypt::hash(body.password.to_string(), 10).unwrap();
    let query = data.query_builder.insert(
        "users",
        &["username", "email", "phone", "password"],
        &[&body.username, &body.email, &body.phone, &password],
    );
    let res = sqlx::query(&query)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    res
}

pub async fn update_user_service(
    id: i32,
    Json(body): Json<CreateUpdateUserSchema>,
) -> Result<sqlx::mysql::MySqlQueryResult, String> {
    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();
    let data: State<Arc<AppState>> = State(app_state);
    let check_user_query = data.query_builder.select_by_id("users", id as u64);
    let user_exists = sqlx::query_as::<_, UserModel>(&check_user_query)
        .fetch_optional(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string())?;

    if user_exists.is_none() {
        return Err(format!("User with id {} does not exist", id));
    }

    update_validation(&body).await?;

    let password = bcrypt::hash(body.password.to_string(), 10).unwrap();
    let query = data.query_builder.update(
        "users",
        id as u64,
        &[
            ("username", &body.username),
            ("email", &body.email),
            ("phone", &body.phone),
            ("password", &password),
        ],
    );
    let res = sqlx::query(&query)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    res
}

pub async fn delete_user_by_id_service(id: i32) -> Result<sqlx::mysql::MySqlQueryResult, String> {
    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();
    let data: State<Arc<AppState>> = State(app_state);
    let check_user_query = data.query_builder.select_by_id("users", id as u64);
    let user_exists = sqlx::query_as::<_, UserModel>(&check_user_query)
        .fetch_optional(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string())?;

    if user_exists.is_none() {
        return Err(format!("User with id {} does not exist", id));
    }

    let query = data.query_builder.delete("users", id as u64);
    let res = sqlx::query(&query)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    res
}
