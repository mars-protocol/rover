use cosmwasm_std::Addr;
use cw721::OwnerOfResponse;
use cw_multi_test::{AppResponse, BasicApp};
use mars_account_nft::msg::QueryMsg;

// Double checking ownership by querying NFT account-nft for correct owner
pub fn assert_owner_is_correct(
    app: &mut BasicApp,
    contract_addr: &Addr,
    user: &Addr,
    token_id: &str,
) {
    let owner_res: OwnerOfResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr,
            &QueryMsg::OwnerOf {
                token_id: token_id.to_string(),
                include_expired: None,
            },
        )
        .unwrap();

    assert_eq!(user.to_string(), owner_res.owner)
}

pub fn get_token_id(res: AppResponse) -> String {
    let attr: Vec<&str> = res
        .events
        .iter()
        .flat_map(|event| &event.attributes)
        .filter(|attr| attr.key == "token_id")
        .map(|attr| attr.value.as_str())
        .collect();

    assert_eq!(attr.len(), 1);
    attr.first().unwrap().to_string()
}
