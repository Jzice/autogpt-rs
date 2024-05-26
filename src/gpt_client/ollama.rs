
use anyhow::Result;
use std::env::var;
use tracing::{debug,info, error};

use ollama_rs::{
    generation::{
        images::Image,
        completion::request::GenerationRequest, 
        options::GenerationOptions
    },
    Ollama,
};

#[derive(Debug, Clone)]
pub struct OllamaClient {
    client: Ollama,
}

impl OllamaClient {
    ///
    pub fn new() -> Self {
        let host = var("OLLAMA_HOST")
            .unwrap_or("localhost".to_string())
            .to_owned();
        let port: u16 = var("OLLAMA_PORT")
            .unwrap_or("11434".to_string())
            .to_owned()
            .parse()
            .unwrap_or(11434);

        Self {
            client: Ollama::new(host, port),
        }
    }
}

impl Default for OllamaClient {
    fn default() -> Self {
        Self {
            client: Ollama::default(),
        }
    }
}

impl OllamaClient {
    /// 
    pub async fn generate(&self, prompt: String) -> Result<String> {
        let model = var("LLAMA_MODEL")
            .unwrap_or("llama3:latest".to_string())
            .to_owned();

        let options = GenerationOptions::default()
            .temperature(0.2)
            .repeat_penalty(1.5)
            .top_k(25)
            .top_p(0.25);

        debug!("[*] ollama prompt {:?}", prompt);
        let resp = match self.client.generate(
            GenerationRequest::new(model, prompt).options(options)
        ).await {
            Ok(response) => {
                info!("[*] ollama generate text {:?}", response.response);
                response.response
            }
            Err(_err) => {
                error!("[*] ollama generate err {:?}", _err);
                Default::default()
            }
        };

        Ok(resp)
    }

    ///
    pub async fn generate_text_from_image(&mut self, prompt: &str, image_path: &str) -> Result<String>{
        let base64 = image_base64::to_base64(image_path);
        let image = Image::from_base64(&base64);

        let resp = match self.client.generate(
                GenerationRequest::new(
                    "llava-llama3:latest".to_string(),
                    prompt.to_string(),
                )
                .add_image(image),
            )
        .await {
            Ok(response) => {
                info!("[*] ollama generate text {:?}", response.response);
                response.response
            }
            Err(_err) => {
                error!("[*] ollama generate err {:?}", _err);
                Default::default()
            }
        };
        Ok(resp)
    }
}
