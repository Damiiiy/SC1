use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)] 
pub struct Property {
    pub id: String,
    pub owner: Addr,
    pub name: String,
    pub description: String,
    pub price: u128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)] 
pub struct Booking {
    pub property_id: String,
    pub guest: Addr,
}

pub const PROPERTY_REGISTRY: Map<&[u8], Property> = Map::new("properties");
pub const BOOKINGS: Map<(&[u8], &Addr), Booking> = Map::new("bookings");
