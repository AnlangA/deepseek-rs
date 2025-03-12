use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponse {
    // 该对话的唯一标识符。
    id: String,
    // 模型生成的 completion 的选择列表。
    pub choices: Vec<Choice>,
    // 创建聊天完成时的 Unix 时间戳（以秒为单位）。
    created: isize,
    // 生成该 completion 的模型名。
    model: String,
    // This fingerprint represents the backend configuration that the model runs with
    system_fingerprint: String,
    // 对象的类型, 其值为 chat.completion。
    object: String,
    // 该对话补全请求的用量信息。
    usage: Option<Usage>,
}

impl ChatResponse {
    pub fn content(&self) -> Vec<&str> {
        self.choices.iter().map(|c| c.content()).collect()
    }
    pub fn role(&self) -> Vec<&str> {
        self.choices.iter().map(|c| c.role()).collect()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    // 模型停止生成 token 的原因。
    // stop：模型自然停止生成，或遇到 stop 序列中列出的字符串。
    // length ：输出长度达到了模型上下文长度限制，或达到了 max_tokens 的限制。
    // content_filter：输出内容因触发过滤策略而被过滤。
    // insufficient_system_resource：系统推理资源不足，生成被打断。
    finish_reason: String,
    // 该 completion 在模型生成的 completion 的选择列表中的索引。
    index: usize,
    // 模型生成的 completion 消息。
    #[serde(rename = "message")]
    response_content: ResponseMessage,
    // 该 choice 的对数概率信息。
    logprobs: Option<Logprobs>,
}

impl Choice {
    pub fn content(&self) -> &str {
        self.response_content.cotent()
    }
    pub fn role(&self) -> &str {
        self.response_content.role()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMessage {
    // 该 completion 的内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    // 仅适用于 deepseek-reasoner 模型。内容为 assistant 消息中在最终答案之前的推理内容。
    reasoning_content: Option<String>,
    // 模型生成的 tool 调用，例如 function 调用。
    tool_calls: Option<Vec<ToolCall>>,
    // 生成这条消息的角色。
    role: String,
}

impl ResponseMessage {
    pub fn cotent(&self) -> &str {
        self.content.as_deref().unwrap_or("没有回复content")
    }

    pub fn role(&self) -> &str {
        self.role.as_str()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    // tool 调用的 ID。
    id: String,
    // tool 的类型。目前仅支持 function。
    #[serde(rename = "type")]
    type_name: String,
    #[serde(rename = "function")]
    response_function: ResponseFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseFunction {
    // 模型调用的 function。
    name: String,
    // 要调用的 function 的参数，由模型生成，格式为 JSON。请注意，模型并不总是生成有效的 JSON，
    // 并且可能会臆造出你函数模式中未定义的参数。在调用函数之前，请在代码中验证这些参数。
    arguments: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Logprobs {
    // 一个包含输出 token 对数概率信息的列表。
    content: LogprobsContent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogprobsContent {
    // 输出的 token。
    tokens: String,
    // 该 token 的对数概率。-9999.0 代表该 token 的输出概率极小，不在 top 20 最可能输出的 token 中。
    logprob: isize,
    // 一个包含该 token UTF-8 字节表示的整数列表。一般在一个 UTF-8 字符被拆分成多个 token 来表示时有用。如果 token 没有对应的字节表示，则该值为 null。
    bytes: Option<isize>,
    // 一个包含在该输出位置上，输出概率 top N 的 token 的列表，以及它们的对数概率。
    // 在罕见情况下，返回的 token 数量可能少于请求参数中指定的 top_logprobs 值。
    top_logprobs: TopLogprobs,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopLogprobs {
    // token。
    token: String,
    // 该 token 的对数概率。
    logprob: isize,
    // 一个包含该 token UTF-8 字节表示的整数列表。一般在一个 UTF-8 字符被拆分成多个 token 来表示时有用。
    // 如果 token 没有对应的字节表示，则该值为 null。
    bytes: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    // 模型 completion 产生的 token 数。
    completion_tokens: isize,
    // 用户 prompt 所包含的 token 数。该值等于 prompt_cache_hit_tokens + prompt_cache_miss_tokens
    prompt_tokens: isize,
    // 用户 prompt 中，命中上下文缓存的 token 数。
    prompt_cache_hit_tokens: isize,
    // 用户 prompt 中，未命中上下文缓存的 token 数。
    prompt_cache_miss_tokens: isize,
    // 请求的 token 数量。
    total_tokens: usize,
    // 该请求中，所有 token 的数量（prompt + completion）
    prompt_tokens_details: PormptTokensDetails,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PormptTokensDetails {
    // 推理模型所产生的思维链 token 数量
    cached_tokens: isize,
}