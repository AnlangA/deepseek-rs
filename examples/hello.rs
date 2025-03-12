use deepseek_rs::{chat::*, http::*, base_types::data::*};
use std::io::{self, Write};
fn get_api_key() -> String {
    print!("请输入您的 API Key: ");
    io::stdout().flush().unwrap();
    
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key).unwrap();
    api_key.trim().to_string()
}

#[tokio::main]
async fn main() {
    let user_message = Message::user_message("你好");
    let (url, ai_request) = ChatRequestBuilder::new()
    .add_message(user_message)
    .model(ModelName::DeepseekChat)
    .build();

    let json = ai_request.to_json().unwrap();

    let api_key = get_api_key();

    println!("url: {}", url);
    println!("json: {}", json);

    let response =  post(&url, json, &api_key).await;

    match process_response(response).await{
        Ok(response) => {
            let data: ChatResponse = serde_json::from_str(&response).unwrap();
            println!("response: {:?}", data.role());
            println!("response: {:?}", data.content());
        }
        Err(err) => {
            println!("error: {}", err);
        }
    }
}