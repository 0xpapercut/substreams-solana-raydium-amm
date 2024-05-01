use serde::{Deserialize, Serialize};
use bincode;
use base64;

pub const LOG_SIZE: usize = 256;

/// LogType enum
#[derive(Debug)]
pub enum LogType {
    Init,
    Deposit,
    Withdraw,
    SwapBaseIn,
    SwapBaseOut,
}

pub enum Log {
    Init(InitLog),
    Deposit(DepositLog),
    Withdraw(WithdrawLog),
    SwapBaseIn(SwapBaseInLog),
    SwapBaseOut(SwapBaseOutLog),
}

impl LogType {
    pub fn from_u8(log_type: u8) -> Self {
        match log_type {
            0 => LogType::Init,
            1 => LogType::Deposit,
            2 => LogType::Withdraw,
            3 => LogType::SwapBaseIn,
            4 => LogType::SwapBaseOut,
            _ => unreachable!(),
        }
    }

    pub fn into_u8(&self) -> u8 {
        match self {
            LogType::Init => 0u8,
            LogType::Deposit => 1u8,
            LogType::Withdraw => 2u8,
            LogType::SwapBaseIn => 3u8,
            LogType::SwapBaseOut => 4u8,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InitLog {
    pub log_type: u8,
    pub time: u64,
    pub pc_decimals: u8,
    pub coin_decimals: u8,
    pub pc_lot_size: u64,
    pub coin_lot_size: u64,
    pub pc_amount: u64,
    pub coin_amount: u64,
    pub market: [u8; 32],
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DepositLog {
    pub log_type: u8,
    // input
    pub max_coin: u64,
    pub max_pc: u64,
    pub base: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    pub pool_lp: u64,
    pub calc_pnl_x: u128,
    pub calc_pnl_y: u128,
    // calc result
    pub deduct_coin: u64,
    pub deduct_pc: u64,
    pub mint_lp: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WithdrawLog {
    pub log_type: u8,
    // input
    pub withdraw_lp: u64,
    // user info
    pub user_lp: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    pub pool_lp: u64,
    pub calc_pnl_x: u128,
    pub calc_pnl_y: u128,
    // calc result
    pub out_coin: u64,
    pub out_pc: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SwapBaseInLog {
    pub log_type: u8,
    // input
    pub amount_in: u64,
    pub minimum_out: u64,
    pub direction: u64,
    // user info
    pub user_source: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    // calc result
    pub out_amount: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SwapBaseOutLog {
    pub log_type: u8,
    // input
    pub max_in: u64,
    pub amount_out: u64,
    pub direction: u64,
    // user info
    pub user_source: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    // calc result
    pub deduct_in: u64,
}

pub fn decode_ray_log(log: &str) -> Log {
    let bytes = base64::decode_config(log, base64::STANDARD).unwrap();
    match LogType::from_u8(bytes[0]) {
        LogType::Init => {
            let log: InitLog = bincode::deserialize(&bytes).unwrap();
            return Log::Init(log);
        }
        LogType::Deposit => {
            let log: DepositLog = bincode::deserialize(&bytes).unwrap();
            return Log::Deposit(log);
        }
        LogType::Withdraw => {
            let log: WithdrawLog = bincode::deserialize(&bytes).unwrap();
            return Log::Withdraw(log);
        }
        LogType::SwapBaseIn => {
            let log: SwapBaseInLog = bincode::deserialize(&bytes).unwrap();
            return Log::SwapBaseIn(log);
        }
        LogType::SwapBaseOut => {
            let log: SwapBaseOutLog = bincode::deserialize(&bytes).unwrap();
            return Log::SwapBaseOut(log);
        }
    }
}
