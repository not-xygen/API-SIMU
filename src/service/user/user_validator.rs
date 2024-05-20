use crate::{
    model::schema::CreateUpdateUserSchema,
    utils::{
        singleton::{get_app_state, AppState},
        validation_chain::ValidationChain,
    },
};
use std::sync::Arc;
use tokio::sync::OnceCell;

static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::const_new();

async fn init_app_state() -> Arc<AppState> {
    get_app_state().await.unwrap()
}

pub async fn create_validation(body: &CreateUpdateUserSchema) -> Result<(), String> {
    let app_state = APP_STATE.get_or_init(init_app_state).await.clone();
    let mut validation = ValidationChain::new(app_state);

    validation.add_rule("username", "required", None);
    validation.add_rule("username", "min_length", Some("3"));
    validation.add_rule("username", "max_length", Some("20"));
    validation.add_rule("email", "required", None);
    validation.add_rule("email", "min_length", Some("5"));
    validation.add_rule("email", "max_length", Some("50"));
    validation.add_rule("email", "email", None);
    validation.add_rule("email", "unique", None);
    validation.add_rule("password", "required", None);
    validation.add_rule("password", "min_length", Some("8"));
    validation.add_rule("password", "contains", None);
    validation.add_rule("phone", "required", None);
    validation.add_rule("phone", "min_length", Some("10"));
    validation.add_rule("phone", "max_length", Some("15"));
    validation.add_rule("phone", "phone", None);

    validation
        .validate("username", &body.username.to_string(), "users")
        .await?;
    validation
        .validate("email", &body.email.to_string(), "users")
        .await?;
    validation
        .validate("password", &body.password.to_string(), "users")
        .await?;
    validation
        .validate("phone", &body.phone.to_string(), "users")
        .await?;

    Ok(())
}

pub async fn update_validation(body: &CreateUpdateUserSchema) -> Result<(), String> {
    let app_state = APP_STATE.get_or_init(init_app_state).await.clone();
    let mut validation = ValidationChain::new(app_state);

    validation.add_rule("username", "required", None);
    validation.add_rule("username", "min_length", Some("3"));
    validation.add_rule("username", "max_length", Some("20"));
    validation.add_rule("email", "required", None);
    validation.add_rule("email", "min_length", Some("5"));
    validation.add_rule("email", "max_length", Some("50"));
    validation.add_rule("email", "email", None);
    validation.add_rule("password", "required", None);
    validation.add_rule("password", "min_length", Some("8"));
    validation.add_rule("password", "contains", None);
    validation.add_rule("phone", "required", None);
    validation.add_rule("phone", "min_length", Some("10"));
    validation.add_rule("phone", "max_length", Some("15"));
    validation.add_rule("phone", "phone", None);

    validation
        .validate("username", &body.username.to_string(), "users")
        .await?;
    validation
        .validate("email", &body.email.to_string(), "users")
        .await?;
    validation
        .validate("password", &body.password.to_string(), "users")
        .await?;
    validation
        .validate("phone", &body.phone.to_string(), "users")
        .await?;

    Ok(())
}
