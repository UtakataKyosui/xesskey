use crate::models::actors;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct WebFingerQuery {
    pub resource: String,
}

#[derive(Serialize)]
pub struct Jrd {
    pub subject: String,
    pub links: Vec<JrdLink>,
}

#[derive(Serialize)]
pub struct JrdLink {
    pub rel: String,
    pub r#type: Option<String>,
    pub href: Option<String>,
}

pub async fn index(
    State(ctx): State<AppContext>,
    Query(params): Query<WebFingerQuery>,
) -> impl IntoResponse {
    let resource = params.resource;
    if !resource.starts_with("acct:") {
        return format::text("Invalid resource").into_response();
    }

    let parts: Vec<&str> = resource.trim_start_matches("acct:").split('@').collect();
    if parts.is_empty() {
        return format::text("Invalid format").into_response();
    }
    let username = parts[0];

    // Find actor
    let actor = actors::Entity::find()
        .filter(actors::Column::Username.eq(username))
        .one(&ctx.db)
        .await;

    match actor {
        Ok(Some(actor)) => {
            let jrd = Jrd {
                subject: resource,
                links: vec![JrdLink {
                    rel: "self".to_string(),
                    r#type: Some("application/activity+json".to_string()),
                    href: Some(actor.uri),
                }],
            };
            Json(jrd).into_response()
        }
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => format::text(&format!("DB Error: {}", e)).into_response(),
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/.well-known/webfinger")
        .add("/", get(index))
}
