use rover::msg::query::CoinValue;

pub fn get_asset(denom: &str, list: &[CoinValue]) -> CoinValue {
    list.iter()
        .find(|item| item.denom == denom)
        .unwrap()
        .clone()
}
