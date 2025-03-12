use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceResponse {
    is_available: bool,
    balance_infos: Vec<BalanceInfos>,
}

impl BalanceResponse {
    pub fn is_available(&self) -> bool {
        self.is_available
    }
    pub fn balance_infos(&self) -> &[BalanceInfos] {
        &self.balance_infos
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceInfos {
    currency: String,
    total_balance: String,
    granted_balance: String,
    topped_up_balance: String,
}

impl BalanceInfos {
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn total_balance(&self) -> &str {
        &self.total_balance
    }
    pub fn granted_balance(&self) -> &str {
        &self.granted_balance
    }
    pub fn topped_up_balance(&self) -> &str {
        &self.topped_up_balance
    }
}