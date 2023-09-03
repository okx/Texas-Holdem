use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Poker {
        user_hands: Vec<String>, // user hands poker which number is two
        board: String,           // the board poker whichi numbe is five
    },
    PokerMulti {
        user_hands: Vec<String>, // user hands poker which number is two
        board: String,           // the board poker whichi numbe is five
        num: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    SimluatePokerWinner {},
}
