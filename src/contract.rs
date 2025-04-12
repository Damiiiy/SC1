use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, Deps, Binary};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{PROPERTY_REGISTRY, Property, Booking, BOOKINGS};
use crate::error::ContractError;


/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:homiq";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/


#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddProperty { id, name, description, price } => {
            let property = Property {
                id: id.clone(),
                owner: info.sender.clone(),
                name,
                description,
                price,
            };
            PROPERTY_REGISTRY.save(deps.storage, id.as_bytes(), &property)?;
            Ok(Response::new().add_attribute("action", "add_property"))
        }
        ExecuteMsg::BookProperty { property_id } => {
            let prop = PROPERTY_REGISTRY.load(deps.storage, property_id.as_bytes()).map_err(|_| ContractError::PropertyNotFound {})?;
            let sent = info.funds.iter().find(|c| c.denom == "uxion");
            if sent.is_none() || sent.unwrap().amount.u128() < prop.price {
                return Err(ContractError::InsufficientFunds {});
            }
            let booking = Booking {
                property_id: property_id.clone(),
                guest: info.sender.clone(),
            };
            BOOKINGS.save(deps.storage, (property_id.as_bytes(), &info.sender), &booking)?;
            Ok(Response::new().add_attribute("action", "book_property"))
        }
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetProperty { id } => {
            let prop = PROPERTY_REGISTRY.load(deps.storage, id.as_bytes())?;
            cosmwasm_std::to_json_binary(&prop)
        }
    }
}















