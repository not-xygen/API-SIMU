use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    model::{model::UserModel, schema::CreateUpdateUserSchema},
    AppState,
};

pub async fn get_all_user_service(
    State(data): State<Arc<AppState>>,
) -> Result<Vec<UserModel>, String> {
    let res = sqlx::query_as::<_, UserModel>(r#"SELECT * FROM users ORDER by id"#)
        .fetch_all(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    res
}

pub async fn get_user_by_id_service(
    State(data): State<Arc<AppState>>,
    id: i32,
) -> Result<UserModel, String> {
    let res = sqlx::query_as::<_, UserModel>(r#"SELECT * FROM users WHERE id = ?"#)
        .bind(id)
        .fetch_one(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    if let Err(_) = res {
        return Err(format!("User with id {} does not exist", id));
    }

    res
}

pub async fn create_user_service(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUpdateUserSchema>,
) -> Result<sqlx::mysql::MySqlQueryResult, String> {
    // Insert
    let password = bcrypt::hash(body.password.to_string(), 10).unwrap();
    let res =
        sqlx::query(r#"INSERT INTO users (username, email, phone, password ) VALUES (?, ?, ?, ?)"#)
            .bind(body.username.to_string())
            .bind(body.email.to_string())
            .bind(body.phone.to_string())
            .bind(password.to_string())
            .execute(&data.db)
            .await
            .map_err(|err: sqlx::Error| err.to_string());

    res
}

pub async fn update_user_service(
    State(data): State<Arc<AppState>>,
    id: i32,
    Json(body): Json<CreateUpdateUserSchema>,
) -> Result<sqlx::mysql::MySqlQueryResult, String> {
    let check_user = sqlx::query_as::<_, UserModel>(r#"SELECT * FROM users WHERE id = ?"#)
        .bind(id)
        .fetch_optional(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string())?;

    if check_user.is_none() {
        return Err(format!("User with id {} does not exist", id));
    }


    // Update
    let password = bcrypt::hash(body.password.to_string(), 10).unwrap();
    let res = sqlx::query(
        r#"UPDATE users SET username = ?, email = ?, phone = ?, password = ? WHERE id = ?"#,
    )
    .bind(body.username.to_string())
    .bind(body.email.to_string())
    .bind(body.phone.to_string())
    .bind(password.to_string())
    .bind(id)
    .execute(&data.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    res
}

pub async fn delete_user_by_id_service(
    State(data): State<Arc<AppState>>,
    id: i32,
) -> Result<sqlx::mysql::MySqlQueryResult, String> {
    let check_user = sqlx::query_as::<_, UserModel>(r#"SELECT * FROM users WHERE id = ?"#)
        .bind(id)
        .fetch_optional(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string())?;

    if check_user.is_none() {
        return Err(format!("User with id {} does not exist", id));
    }

    let res = sqlx::query(r#"DELETE FROM users WHERE id = ?"#)
        .bind(id)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    res
}
