use cosmwasm_schema::serde::{de::DeserializeOwned, Serialize};
use mars_rover_health_types::BorrowTarget;
use mars_rover_health_types::HealthValuesResponse;
use wasm_bindgen::prelude::*;

use crate::HealthComputer;

#[wasm_bindgen]
pub fn compute_health_js(health_computer: JsValue) -> JsValue {
    let c: HealthComputer = deserialize(health_computer);
    let health = c.compute_health().unwrap();
    let health_response: HealthValuesResponse = health.into();
    serialize(health_response)
}

#[wasm_bindgen]
pub fn max_withdraw_estimate_js(health_computer: JsValue, withdraw_denom: JsValue) -> JsValue {
    let c: HealthComputer = deserialize(health_computer);
    let denom: String = deserialize(withdraw_denom);
    let max = c.max_withdraw_amount_estimate(&denom).unwrap();
    serialize(max)
}

#[wasm_bindgen]
pub fn max_borrow_estimate_js(
    health_computer: JsValue,
    borrow_denom: JsValue,
    target: JsValue,
) -> JsValue {
    let c: HealthComputer = deserialize(health_computer);
    let denom: String = deserialize(borrow_denom);
    let target: BorrowTarget = deserialize(target);
    let max = c.max_borrow_amount_estimate(&denom, target).unwrap();
    serialize(max)
}

pub fn serialize<T: Serialize>(val: T) -> JsValue {
    serde_wasm_bindgen::to_value(&val).unwrap()
}

pub fn deserialize<T: DeserializeOwned>(val: JsValue) -> T {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    serde_wasm_bindgen::from_value(val).unwrap()
}
