use super::super::base_types::data::*;
use serde::{Deserialize, Serialize};

/// chat类型请求
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatRequest {
    // 对话的消息列表。
    messages: Vec<Message>,
    // 使用的模型的 ID。您可以使用 deepseek-chat。
    model: String,
    // 介于 -2.0 和 2.0 之间的数字。如果该值为正，
    // 那么新 token 会根据其在已有文本中的出现频率受到相应的惩罚，降低模型重复相同内容的可能性。
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f64>,
    // 介于 1 到 8192 间的整数，限制一次请求中模型生成 completion 的最大 token 数。
    // 输入 token 和输出 token 的总长度受模型的上下文长度的限制。
    // 如未指定 max_tokens参数，默认使用 4096。
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<usize>,
    // 介于 -2.0 和 2.0 之间的数字。如果该值为正，那么新 token 会根据其是否已在已有文本中出现受到相应的惩罚，
    // 从而增加模型谈论新主题的可能性。
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f64>,
    // 一个 object，指定模型必须输出的格式。
    // 设置为 { "type": "json_object" } 以启用 JSON 模式，该模式保证模型生成的消息是有效的 JSON。
    // 注意: 使用 JSON 模式时，你还必须通过系统或用户消息指示模型生成 JSON。否则，
    // 模型可能会生成不断的空白字符，直到生成达到令牌限制，
    // 从而导致请求长时间运行并显得“卡住”。此外，
    // 如果 finish_reason="length"，这表示生成超过了 max_tokens 
    // 或对话超过了最大上下文长度，消息内容可能会被部分截断。
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<RespinseFormat>,
    // 一个 string 或最多包含 16 个 string 的 list，在遇到这些词时，API 将停止生成更多的 token。
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    // 如果设置为 True，将会以 SSE（server-sent events）的形式以流式发送消息增量。消息流以 data: [DONE] 结尾。
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    // 采样温度，介于 0 和 2 之间。更高的值，如 0.8，会使输出更随机，而更低的值，如 0.2，会使其更加集中和确定。 
    // 我们通常建议可以更改这个值或者更改 top_p，但不建议同时对两者进行修改。
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    // 作为调节采样温度的替代方案，模型会考虑前 top_p 概率的 token 的结果。所以 0.1 就意味着只有包括在最高 10% 概率中的 token 会被考虑。 
    // 我们通常建议修改这个值或者更改 temperature，但不建议同时对两者进行修改
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,
    // 模型可能会调用的 tool 的列表。目前，仅支持 function 作为工具。使用此参数来提供以 JSON 作为输入参数的 function 列表。
    // 最多支持 128 个 function。
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
}

impl ChatRequest {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
    pub fn add_messages(&mut self, messages: Vec<Message>) {
        self.messages.extend(messages);
    }
}

/// chat类型请求构建器
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatRequestBuilder{
    // 对话的消息列表。
    messages: Vec<Message>,
    // 使用的模型的 ID。您可以使用 deepseek-chat。
    model: ModelName,
    // 介于 -2.0 和 2.0 之间的数字。如果该值为正，
    // 那么新 token 会根据其在已有文本中的出现频率受到相应的惩罚，降低模型重复相同内容的可能性。
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f64>,
    // 介于 1 到 8192 间的整数，限制一次请求中模型生成 completion 的最大 token 数。
    // 输入 token 和输出 token 的总长度受模型的上下文长度的限制。
    // 如未指定 max_tokens参数，默认使用 4096。
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<usize>,
    // 介于 -2.0 和 2.0 之间的数字。如果该值为正，那么新 token 会根据其是否已在已有文本中出现受到相应的惩罚，
    // 从而增加模型谈论新主题的可能性。
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f64>,
    // 一个 object，指定模型必须输出的格式。
    // 设置为 { "type": "json_object" } 以启用 JSON 模式，该模式保证模型生成的消息是有效的 JSON。
    // 注意: 使用 JSON 模式时，你还必须通过系统或用户消息指示模型生成 JSON。否则，
    // 模型可能会生成不断的空白字符，直到生成达到令牌限制，
    // 从而导致请求长时间运行并显得“卡住”。此外，
    // 如果 finish_reason="length"，这表示生成超过了 max_tokens 
    // 或对话超过了最大上下文长度，消息内容可能会被部分截断。
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<RespinseFormat>,
    // 一个 string 或最多包含 16 个 string 的 list，在遇到这些词时，API 将停止生成更多的 token。
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    // 如果设置为 True，将会以 SSE（server-sent events）的形式以流式发送消息增量。消息流以 data: [DONE] 结尾。
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    // 采样温度，介于 0 和 2 之间。更高的值，如 0.8，会使输出更随机，而更低的值，如 0.2，会使其更加集中和确定。 
    // 我们通常建议可以更改这个值或者更改 top_p，但不建议同时对两者进行修改。
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    // 作为调节采样温度的替代方案，模型会考虑前 top_p 概率的 token 的结果。所以 0.1 就意味着只有包括在最高 10% 概率中的 token 会被考虑。 
    // 我们通常建议修改这个值或者更改 temperature，但不建议同时对两者进行修改
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,
    // 模型可能会调用的 tool 的列表。目前，仅支持 function 作为工具。使用此参数来提供以 JSON 作为输入参数的 function 列表。
    // 最多支持 128 个 function。
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
}

impl ChatRequestBuilder {
    pub fn new() -> Self {
        ChatRequestBuilder {
            messages: Vec::new(),
            model: ModelName::DeepseekChat,
            frequency_penalty: None,
            max_tokens: None,
            presence_penalty: None,
            response_format: None,
            stop: None,
            stream: None,
            temperature: None,
            top_p: None,
            tools: None
        }
    }
    pub fn add_message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }
    
    pub fn model(mut self, model: ModelName) -> Self {
        self.model = model;
        self
    }
    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }
    pub fn max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
    pub fn presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }
    pub fn response_format(mut self, response_format: RespinseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }
    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }
    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }
    pub fn tools(mut self, tools: Tool) -> Self {
        self.tools.get_or_insert_with(Vec::new).push(tools);
        self
    }
    pub fn build(self) -> (String, ChatRequest) {
        let mut base_url = String::new();
        if self.model == ModelName::DeepseekChat{
            base_url = format!("{}https://api.deepseek.com/chat",base_url);
        }else if self.model == ModelName::DeepseekReasoner{
            base_url = format!("{}https://api.deepseek.com/chat",base_url);
        }
        base_url = format!("{}/completions",base_url);
        
        (
            base_url,
            ChatRequest {
                messages: self.messages,
                model: self.model.to_string(),
                frequency_penalty: self.frequency_penalty,
                max_tokens: self.max_tokens,
                presence_penalty: self.presence_penalty,
                response_format: self.response_format,
                stop: self.stop,
                stream: self.stream,
                temperature: self.temperature,
                top_p: self.top_p,
                tools: self.tools,
            }
        )
    }
}

/// 系统消息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message{
    //消息的内容
    content: String,
    //该消息的发起角色,其值为 `system`
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    //可以选填的参与者的名称，为模型提供信息以区分相同角色的参与者。
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<bool>,
    //(Beta) 用于 deepseek-reasoner 模型在对话前缀续写功能下，作为最后一条 assistant 思维链内容的输入。
    //使用此功能时，prefix 参数必须设置为 true。
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_content: Option<String>,
    // 此消息所响应的 tool call 的 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

impl Message {
    pub fn system_message(content: &str) -> Self {
        Message {
            content: String::from(content),
            role: String::from("system"),
            name: None,
            prefix: None,
            reasoning_content: None,
            tool_call_id: None,
        }
    }
    pub fn user_message(content: &str) -> Self {
        Message {
            content: String::from(content),
            role: String::from("user"),
            name: None,
            prefix: None,
            reasoning_content: None,
            tool_call_id: None,
        }
    }
    pub fn assistant_message(content: &str) -> Self {
        Message {
            content: String::from(content),
            role: String::from("assistant"),
            name: None,
            prefix: None,
            reasoning_content: None,
            tool_call_id: None,
        }
    }
    // 可以选填的参与者的名称，为模型提供信息以区分相同角色的参与者。
    pub fn name(mut self, name: &str) -> Self {
        if self.role == "system" || self.role == "user" || self.role == "assistant"{
            self.name = Some(String::from(name));
        }
        self
    }
    // role 类型为 assistant 时，是否在消息内容前添加 “Assistant:” 前缀。
    pub fn prefix(mut self, prefix: bool) -> Self {
        if self.role == "assistant" {
            self.prefix = Some(prefix);
        }
        self
    }
    // (Beta) role 类型为 assistant 时，用于 deepseek-reasoner 模型在对话前缀续写功能下，作为最后一条 assistant 思维链内容的输入。
    pub fn reasoning_content(mut self, content: &str) -> Self {
        if self.role == "assistant" {
            self.reasoning_content = Some(String::from(content));
        }
        self
    }
    // 此消息所响应的 tool call 的 ID。
    pub fn tool_call_id(mut self, id: &str) -> Self {
        if self.role == "tool"{
            self.tool_call_id = Some(String::from(id));
        }
        self
    }
    
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespinseFormat{
    // ai回复的格式，可选值为 `text` 和 `json_object`。
    #[serde(rename = "type")]
    format_type: String,
}

impl RespinseFormat {
    // 设置ai回复为文本格式
    pub fn text() -> Self {
        RespinseFormat {
            format_type: String::from("text"),
        }
    }
    // 设置ai回复为json格式
    pub fn json_object() -> Self {
        RespinseFormat {
            format_type: String::from("json_object"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool{
    // 类型，为： “function”。
    #[serde(rename = "type")]
    fun_type: String,
    // 函数的参数。
    function: Option<Function>,
}

impl Tool {
    pub fn function() -> Self {
        Tool {
            fun_type: String::from("function"),
            function: Some(Function::new()),
        }
    }
    pub fn function_description(mut self, description: &str) -> Self {
        if self.fun_type == "function" {
            self.function.as_mut().unwrap().description(description);
        }
        self
    }
    pub fn function_name(mut self, name: &str) -> Self {
        if self.fun_type == "function" {
            self.function.as_mut().unwrap().name(name);
        }
        self
    }
    pub fn function_parameters(mut self, parameters: serde_json::Value) -> Self {
        if self.fun_type == "function" {
            self.function.as_mut().unwrap().parameters(parameters);
        }
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function{
    // function 的功能描述，供模型理解何时以及如何调用该 function。
    description: String,
    // 要调用的 function 名称。必须由 a-z、A-Z、0-9 字符组成，或包含下划线和连字符，最大长度为 64 个字符。
    name: String,
    // function 的输入参数，以 JSON Schema 对象描述。请参阅 Function Calling 指南获取示例，
    // 并参阅JSON Schema 参考了解有关格式的文档。省略 parameters 会定义一个参数列表为空的 function。
    parameters: serde_json::Value,
}

impl Function {
    pub fn new() -> Self {
        Function {
            description: String::from("function one"),
            name: String::from("function_one"),
            parameters: serde_json::Value::Null,
        }
    }
    pub fn description(&mut self, description: &str) {
        self.description = description.to_string();
    }
    pub fn name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn parameters(&mut self, parameters: serde_json::Value) {
        self.parameters = parameters;
    }
}