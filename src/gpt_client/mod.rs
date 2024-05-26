use anyhow::Result;

// /// traite of GPT client
// pub trait GPTClient {
//     /// generate text by prompt
//     async fn generate(&self, prompt: String) -> Result<String>;
//     /// 
//     async fn generate_text_from_image(&mut self, prompt: &str, image_path: &str) -> Result<String>;
// }

pub mod gemini;
pub mod ollama;

#[derive(Debug, Clone)]
pub enum GPTType {
    Ollama(ollama::OllamaClient),
    Gemini(gemini::GeminiClient),
}

impl Default for GPTType {
    fn default() -> Self {
        GPTType::Ollama(ollama::OllamaClient::default())
    }
}

impl PartialEq for GPTType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            GPTType::Ollama(_) if matches!(other, GPTType::Ollama(_)) => true,
            GPTType::Gemini(_) if matches!(other, GPTType::Gemini(_))=> true,
            _ => false,
        }
    }
}

impl GPTType {
    pub fn new(name: &str) -> Self {
        match name {
            "ollama" => GPTType::Ollama(ollama::OllamaClient::default()),
            "gemini" => GPTType::Gemini(gemini::GeminiClient::default()),
            _ => GPTType::default(),
        }
    }

    /// generate
    pub async fn generate(&self, prompt: String) -> Result<String> {
        match self {
            GPTType::Ollama(gpt) => gpt.generate(prompt).await,
            GPTType::Gemini(gpt) => gpt.generate(prompt).await,
        }
    }

    /// generate text from
    pub async fn generate_text_from_image(&mut self, prompt: &str, image_path: &str) -> Result<String> {
        match self {
            GPTType::Ollama(gpt) => gpt.generate_text_from_image(prompt, image_path).await,
            GPTType::Gemini(gpt) => gpt.generate_text_from_image(prompt, image_path).await,
        }
    }
}

