use cosmwasm_std::{Decimal, StdError, StdResult, Uint128};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use wasm_bindgen::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub denom: String,
    pub price: Decimal,
    pub amount: Uint128,
}

#[wasm_bindgen]
pub fn total_value_js(val: JsValue) -> JsValue {
    let positions = deserialize_from_js(val);
    let res = total_value_rust(positions).unwrap();
    serialize_for_js(res)
}

pub fn total_value_rust(positions: Vec<Position>) -> StdResult<Decimal> {
    let mut total_value = Decimal::zero();
    for p in positions {
        let amount_dec = Decimal::from_atomics(p.amount, 0)
            .map_err(|_| StdError::generic_err("conversion error"))?;
        let value = p.price.checked_mul(amount_dec)?;
        total_value = total_value.checked_add(value)?;
    }
    Ok(total_value)
}

#[wasm_bindgen]
pub fn add_amounts_js(val: JsValue) -> JsValue {
    let positions = deserialize_from_js(val);
    let res = add_amounts_rust(positions);
    serialize_for_js(res)
}

pub fn add_amounts_rust(positions: Vec<Position>) -> Uint128 {
    positions.iter().fold(Uint128::zero(), |total, position| {
        total.add(position.amount)
    })
}

pub fn deserialize_from_js<T: DeserializeOwned>(val: JsValue) -> T {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    serde_wasm_bindgen::from_value(val).unwrap()
}

pub fn serialize_for_js<T: Serialize>(val: T) -> JsValue {
    serde_wasm_bindgen::to_value(&val).unwrap()
}

#[wasm_bindgen]
pub fn greet(s: &str) -> String {
    format!("Hello, {s}!")
}
