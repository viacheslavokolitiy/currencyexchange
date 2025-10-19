use crate::datasource::api_models::CreatedUser;
use crate::datasource::models::User;

pub fn map_user_to_network_model(
    db_user: &User,
) -> CreatedUser {
    CreatedUser::new(
        db_user.user_id,
        db_user.username.clone(),
        db_user.email.clone(),
        db_user.firstname.clone(),
        db_user.middlename.clone(),
        db_user.lastname.clone(),
        db_user.created_at,
        db_user.updated_at
    )
}