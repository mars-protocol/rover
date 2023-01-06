use cosmwasm_std::Coin;

use mars_coin::Coin256;

use crate::msg::execute::ActionCoin;

pub trait Stringify {
    fn to_string(&self) -> String;
}

pub trait ToCoins {
    fn to_coins(&self) -> Vec<Coin256>;
}

pub trait FallbackStr {
    fn fallback(&self, fallback: &str) -> String;
}

impl FallbackStr for String {
    fn fallback(&self, fallback: &str) -> String {
        match self {
            s if !s.is_empty() => s.clone(),
            _ => fallback.to_string(),
        }
    }
}

pub trait ToDenoms {
    fn to_denoms(&self) -> Vec<&str>;
}

impl ToDenoms for Vec<Coin> {
    fn to_denoms(&self) -> Vec<&str> {
        self.iter().map(|c| c.denom.as_str()).collect()
    }
}

impl ToDenoms for Vec<ActionCoin> {
    fn to_denoms(&self) -> Vec<&str> {
        self.iter().map(|c| c.denom.as_str()).collect()
    }
}
