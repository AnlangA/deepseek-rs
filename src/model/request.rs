//! # 查找可用模型

pub struct ModelRequest;

impl ModelRequest {
    pub fn url() -> String {
        "https://api.deepseek.com/models".to_string()
    }
}