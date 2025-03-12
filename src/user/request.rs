//! 查看余额

pub struct BalanceRequest;


impl BalanceRequest {
    pub fn url() -> String {
        "https://api.deepseek.com/user/balance".to_string()
    }
}