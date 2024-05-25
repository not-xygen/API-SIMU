use crate::{
    model::schema::CreateUpdateUserSchema,
    utils::{
        singleton::{init_app_state, AppState},
        validation_chain::ValidationChain,
    },
};
use std::sync::Arc;
use tokio::sync::OnceCell;

static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::const_new();

pub async fn create_validation(body: &CreateUpdateUserSchema) -> Result<(), String> {
    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();

    let fields_values = [
        ("username", body.username.as_str()),
        ("email", body.email.as_str()),
        ("password", body.password.as_str()),
        ("phone", body.phone.as_str()),
    ];

    let mut validation_chain = ValidationChain::new(app_state); // Add 'mut' here
    validation_chain
        .add_rule("username", "required", None)
        .add_rule("username", "min_length", Some("3"))
        .add_rule("username", "max_length", Some("20"))
        .add_rule("email", "required", None)
        .add_rule("email", "min_length", Some("5"))
        .add_rule("email", "max_length", Some("50"))
        .add_rule("email", "email", None)
        .add_rule("email", "unique", None)
        .add_rule("password", "required", None)
        .add_rule("password", "min_length", Some("8"))
        .add_rule("password", "contains", None)
        .add_rule("phone", "required", None)
        .add_rule("phone", "min_length", Some("10"))
        .add_rule("phone", "max_length", Some("15"))
        .add_rule("phone", "phone", None);
    validation_chain
        .validate_fields_recursive(&fields_values, "users")
        .await
}

pub async fn update_validation(body: &CreateUpdateUserSchema) -> Result<(), String> {
    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();

    let fields_values = [
        ("username", body.username.as_str()),
        ("email", body.email.as_str()),
        ("password", body.password.as_str()),
        ("phone", body.phone.as_str()),
    ];

    let mut validation_chain = ValidationChain::new(app_state); // Add 'mut' here
    validation_chain
        .add_rule("username", "required", None)
        .add_rule("username", "min_length", Some("3"))
        .add_rule("username", "max_length", Some("20"))
        .add_rule("email", "required", None)
        .add_rule("email", "min_length", Some("5"))
        .add_rule("email", "max_length", Some("50"))
        .add_rule("email", "email", None)
        .add_rule("password", "required", None)
        .add_rule("password", "min_length", Some("8"))
        .add_rule("password", "contains", None)
        .add_rule("phone", "required", None)
        .add_rule("phone", "min_length", Some("10"))
        .add_rule("phone", "max_length", Some("15"))
        .add_rule("phone", "phone", None);
    validation_chain
        .validate_fields_recursive(&fields_values, "users")
        .await
}
