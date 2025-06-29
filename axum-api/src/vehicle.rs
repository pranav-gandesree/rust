use axum::{debug_handler, Json};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: Option<String>
}

#[debug_handler]
pub async fn get_vehicle() -> Json<Vehicle> {
    println!("get_vehicle called from axum from new file vehicle.rs");
    Json::from(
        Vehicle {
            manufacturer: "Toyota".to_string(),
            model: "Corolla".to_string(),
            year: 2020,
            id: Some(uuid::Uuid::new_v4().to_string())
        })
}   

pub async fn post_vehicle() -> &'static str {
    "Vehicle data posted"
}