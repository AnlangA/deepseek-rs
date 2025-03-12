use serde::{Serialize, Deserialize};
use std::fmt;

// 模型名称
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ModelName {
    DeepseekChat,
    DeepseekReasoner
}

// 将模型名称由枚举转为字符串，方便使用
impl fmt::Display for ModelName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModelName::DeepseekChat => write!(f, "deepseek-chat"),
            ModelName::DeepseekReasoner => write!(f, "deepseek-reasoner"),
        }
    }
}

// 模型字符串与枚举的互相转换
impl From<ModelName> for &str {
    fn from(model_name: ModelName) -> Self {
        match model_name {
            ModelName::DeepseekChat => "deepseek-chat",
            ModelName::DeepseekReasoner => "deepseek-reasoner",
        }
    }
}

