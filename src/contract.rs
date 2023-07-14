#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, to_vec, Binary, Deps, DepsMut, Env, MessageInfo, Response, Storage,
};
use cosmwasm_storage::PrefixedStorage;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::Constants;
use poker::{box_cards, cards, Card, Eval, Evaluator};

pub const PREFIX_CONFIG: &[u8] = b"config";

pub const KEY_CONSTANTS: &[u8] = b"constants";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Check name, symbol, decimals
    if !is_valid_name(&msg.name) {
        return Err(ContractError::NameWrongFormat {});
    }

    let mut config_store = PrefixedStorage::new(deps.storage, PREFIX_CONFIG);
    let constants = to_vec(&Constants { name: msg.name })?;
    config_store.set(KEY_CONSTANTS, &constants);

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Poker { user_hands, board } => try_call_to_poker(deps, env, user_hands, board),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::SimluatePokerWinner {} => Ok(to_binary("data")?),
    }
}

fn try_call_to_poker(
    _: DepsMut,
    _env: Env,
    user_hands: Vec<String>,
    board: String,
) -> Result<Response, ContractError> {
    if user_hands.len() < 1 {
        return Err(ContractError::GetWinnerFaiile(
            "the number of user_hands is one at least ".to_string(),
        ));
    }

    let mut winner = 0_u128;
    let mut winner_cards: Eval = get_best_eval_with_hands_board(&user_hands[0], &board)?;

    let mut index = 1_u128;
    for iter in user_hands.iter().skip(1) {
        let current_cars = get_best_eval_with_hands_board(iter, &board)?;
        if current_cars.is_better_than(winner_cards.clone()) {
            winner_cards = current_cars;
            winner = index;
        }
        index = index + 1;
    }

    let winner_hands = match user_hands.get(winner as usize) {
        Some(s) => s,
        None => {
            return Err(ContractError::GetWinnerFaiile("unknown error".to_string()));
        }
    };
    Ok(Response::new()
        .add_attribute("winner", winner.to_string())
        .add_attribute("cards", winner_cards.to_string())
        .add_attribute("hands", winner_hands)
        .set_data(winner.to_be_bytes()))
}

fn get_best_eval_with_hands_board(hands: &String, board: &String) -> Result<Eval, ContractError> {
    let eval = Evaluator::new();
    let board: Vec<Card> = match cards!(board.as_str()).try_collect() {
        Ok(cards) => cards,
        Err(_) => {
            return Err(ContractError::GetWinnerFaiile(
                "couldn't parse board cards".to_string(),
            ));
        }
    };

    let hands: Vec<Card> = match cards!(hands.as_str()).try_collect() {
        Ok(cards) => cards,
        Err(_) => {
            return Err(ContractError::GetWinnerFaiile(
                "couldn't parse hand bcards".to_string(),
            ));
        }
    };

    let result = match eval.evaluate(box_cards!(board, hands)) {
        Ok(e) => e,
        Err(_) => {
            return Err(ContractError::GetWinnerFaiile(
                "couldn't evaluate hand cards".to_string(),
            ));
        }
    };

    Ok(result)
}

fn is_valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() < 3 || bytes.len() > 30 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Addr, Env, MessageInfo, Timestamp};

    fn mock_env_height(signer: &str, height: u64, time: u64) -> (Env, MessageInfo) {
        let mut env = mock_env();
        let info = mock_info(signer, &[]);
        env.block.height = height;
        env.block.time = Timestamp::from_seconds(time);
        (env, info)
    }

    mod poker {
        use super::*;

        fn make_instantiate_msg() -> InstantiateMsg {
            InstantiateMsg {
                name: "Cash Token".to_string(),
            }
        }

        #[test]
        fn works() {
            let mut deps = mock_dependencies();
            let instantiate_msg = make_instantiate_msg();
            let (env, info) = mock_env_height("creator", 450, 550);
            let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
            assert_eq!(0, res.messages.len());
            let owner = "addr0000";

            // Set approval
            let poker_msg = ExecuteMsg::Poker {
                user_hands: vec![
                    "Kc 4c".to_string(),
                    "3s 3h".to_string(),
                    "5s 5h".to_string(),
                    "Tc Ac".to_string(),
                    "3d Ah".to_string(),
                    "Th Ad".to_string(),
                    "Kh Th".to_string(),
                    "2s 2c".to_string(),
                    "7c 6c".to_string(),
                ],
                board: "3c 5c As Jc Qh".to_string(),
            };
            let (env, info) = mock_env_height(&owner.clone(), 450, 550);
            let approve_result = execute(deps.as_mut(), env, info, poker_msg).unwrap();
            assert_eq!(approve_result.messages.len(), 0);
            print!("reslut{:?}", approve_result)
        }
    }

    mod query {
        use super::*;

        fn address(index: u8) -> Addr {
            match index {
                0 => Addr::unchecked("addr0000".to_string()), // contract instantiateializer
                1 => Addr::unchecked("addr1111".to_string()),
                2 => Addr::unchecked("addr4321".to_string()),
                3 => Addr::unchecked("addr5432".to_string()),
                4 => Addr::unchecked("addr6543".to_string()),
                _ => panic!("Unsupported address index"),
            }
        }

        fn make_instantiate_msg() -> InstantiateMsg {
            InstantiateMsg {
                name: "Cash Token".to_string(),
            }
        }

        #[test]
        fn can_query_balance_of_existing_address() {
            let mut deps = mock_dependencies();
            let instantiate_msg = make_instantiate_msg();
            let (env, info) = mock_env_height(&address(0).as_str(), 450, 550);
            let res = instantiate(deps.as_mut(), env.clone(), info, instantiate_msg).unwrap();
            assert_eq!(0, res.messages.len());
            let query_msg = QueryMsg::SimluatePokerWinner {};
            let query_result = query(deps.as_ref(), env, query_msg).unwrap();
            assert_eq!(query_result.as_slice(), b"{\"balance\":\"11\"}");
        }
    }
}
