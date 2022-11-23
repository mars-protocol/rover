use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;
use cw721_base::InstantiateMsg as ParentInstantiateMsg;

#[cw_serde]
pub struct InstantiateMsg {
    //--------------------------------------------------------------------------------------------------
    // Extended and overridden messages
    //--------------------------------------------------------------------------------------------------
    /// Address of credit manager. Used to query account balances before allowing burns.
    pub credit_manager: String,
    /// The maximum amount of Debts + Collaterals for an account before burns are disallowed
    /// for the NFT. Meant to prevent accidental account deletions.
    pub max_value_for_burn: Decimal,

    //--------------------------------------------------------------------------------------------------
    // Base cw721 messages
    //--------------------------------------------------------------------------------------------------
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,
    /// The minter is the only one who can create new NFTs.
    /// This is designed for a base NFT that is controlled by an external program
    /// or contract. You will likely replace this with custom logic in custom NFTs
    pub minter: String,
}

impl From<InstantiateMsg> for ParentInstantiateMsg {
    fn from(msg: InstantiateMsg) -> Self {
        Self {
            name: msg.name,
            symbol: msg.symbol,
            minter: msg.minter,
        }
    }
}
