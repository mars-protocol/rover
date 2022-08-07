pub fn query_balance(app: &mut App, addr: &Addr, denom: &str) -> Coin {
    app.wrap().query_balance(addr, denom).unwrap()
}

pub fn get_asset(denom: &str, list: &Vec<CoinValue>) -> CoinValue {
    list.iter()
        .find(|item| item.denom == denom)
        .unwrap()
        .clone()
}
