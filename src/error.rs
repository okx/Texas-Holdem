use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Name is not in the expected format (3-30 UTF-8 bytes)")]
    NameWrongFormat {},

    #[error("Geting winner is failed:{0}")]
    GetWinnerFaiile(String),
}
