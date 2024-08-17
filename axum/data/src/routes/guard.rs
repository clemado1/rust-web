use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::users::{self, Entity as Users};

pub async fn guard(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .token()
        .to_owned();

    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = Users::find()
        .filter(users::Column::Token.eq(token))
        .one(database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(user) = user else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
