use crate::models::actors;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;

pub async fn get_actor(
    Path(username): Path<String>,
    State(ctx): State<AppContext>,
) -> impl IntoResponse {
    let actor = actors::Entity::find()
        .filter(actors::Column::Username.eq(&username))
        .one(&ctx.db)
        .await;

    match actor {
        Ok(Some(actor)) => {
            // Construct basic Person object
            // TODO: Use activitystreams crate for type safety
            let person = json!({
                "@context": [
                    "https://www.w3.org/ns/activitystreams",
                    "https://w3id.org/security/v1"
                ],
                "id": actor.uri,
                "type": "Person",
                "preferredUsername": actor.username,
                "inbox": actor.inbox_url,
                "outbox": actor.outbox_url,
                "publicKey": {
                    "id": format!("{}#main-key", actor.uri),
                    "owner": actor.uri,
                    "publicKeyPem": actor.public_key
                }
            });

            // Content-Type should be application/activity+json
            // axum::Json sets application/json. We might need custom response builder.
            // But for now, Json is fine or use explicit response.
            (
                [(
                    axum::http::header::CONTENT_TYPE,
                    "application/activity+json",
                )],
                Json(person),
            )
                .into_response()
        }
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => format::text(&format!("DB Error: {}", e)).into_response(),
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("users")
        .add("/:username", get(get_actor))
}
