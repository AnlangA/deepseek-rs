use deepseek_rs::model::{ModelRequest, ModelResponse};
use deepseek_rs::http::{get, process_response};
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
    let url = ModelRequest::url();

    println!("url: {}", url);
    
    let api_key = get_api_key();

    let response =  get(&url, &api_key).await;
    match process_response(response).await{
        Ok(response) => {
            let data: ModelResponse = serde_json::from_str(&response).unwrap();
            println!("response: {:?}", data);
        }
        Err(err) => {
            println!("error: {}", err);
        }
    }
}
