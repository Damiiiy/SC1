use cosmwasm_schema::{cw_serde, QueryResponses};


#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddProperty { id: String, name: String, description: String, price: u128 },
    BookProperty { property_id: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::state::Property)]
    GetProperty { id: String },
}

