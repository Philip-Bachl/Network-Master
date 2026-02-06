use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::DeviceKind};

#[get("/device_kind")]
pub async fn read_device_kind_all(
    masterbase: &State<Masterbase>,
) -> Result<Json<Vec<DeviceKind>>, String> {
    sqlx::query_as(
        "
            SELECT * FROM dk_device_kind
        ",
    )
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|err| err.to_string())
}

#[post("/device_kind", data = "<device_kind>")]
pub async fn create_device_kind(
    masterbase: &State<Masterbase>,
    device_kind: Json<DeviceKind>,
) -> Status {
    sqlx::query(
        "
            INSERT INTO dk_device_kind VALUES (NULL, $1, $2)
        ",
    )
    .bind(&device_kind.dk_name)
    .bind(&device_kind.dk_kommentar)
    .execute(&masterbase.connection_pool)
    .await
    .map_or_else(|_| Status::InternalServerError, |_| Status::Created)
}

#[put("/device_kind", data = "<device_kind>")]
pub async fn update_device_kind(
    masterbase: &State<Masterbase>,
    device_kind: Json<DeviceKind>,
) -> Status {
    sqlx::query(
        "
            UPDATE dk_device_kind
            SET
            dk_name = $1,
            dk_kommentar = $2
            WHERE dk_id = $3
        ",
    )
    .bind(&device_kind.dk_name)
    .bind(&device_kind.dk_kommentar)
    .bind(device_kind.dk_id)
    .execute(&masterbase.connection_pool)
    .await
    .map_or_else(|_| Status::InternalServerError, |_| Status::Accepted)
}

#[derive(Deserialize)]
pub struct DeleteDeviceKind {
    dk_id: i32,
}

#[delete("/device_kind", data = "<delete_device_kind>")]
pub async fn delete_device_kind(
    masterbase: &State<Masterbase>,
    delete_device_kind: Json<DeleteDeviceKind>,
) -> Status {
    sqlx::query(
        "
            DELETE FROM dk_device_kind
            WHERE dk_id = $1
        ",
    )
    .bind(delete_device_kind.dk_id)
    .execute(&masterbase.connection_pool)
    .await
    .map_or_else(|_| Status::InternalServerError, |_| Status::Ok)
}
