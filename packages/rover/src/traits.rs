use cosmwasm_std::Coin;

pub trait Stringify {
    fn to_string(&self) -> String;
}

pub trait Denoms {
    fn to_denoms(&self) -> Vec<&str>;
}

pub trait Coins {
    fn to_coins(&self) -> Vec<Coin>;
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
