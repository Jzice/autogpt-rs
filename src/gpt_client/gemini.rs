// 
use anyhow::Result;
use gems::Client as GemsClient;
use std::env::var;
// use super::GPTClient;
use gems::utils::load_and_encode_image;

use getimg::client::Client as ImgClient;
use getimg::utils::save_image;
use tracing::{info, debug};

use crate::common::utils::strip_code_blocks;

/// gemini client
#[derive(Debug, Clone)]
pub struct GeminiClient {
    /// gemini client
    client: GemsClient,
    //img_client: ImgClient,
}

impl GeminiClient {
    /// gemini
    pub fn new() -> Self {
        let model = var("GEMINI_MODEL")
            .unwrap_or("gemini-pro".to_string())
            .to_owned();
        let api_key = var("GEMINI_API_KEY")
            .unwrap_or_default()
            .to_owned();

        let getimg_api_key = var("GETIMG_API_KEY").unwrap_or_default().to_owned();
        let getimg_model = var("GETIMG__MODEL")
            .unwrap_or("lcm-realistic-vision-v5-1".to_string())
            .to_owned();

        Self {
            client: GemsClient::new(&api_key, &model),
            //img_client: ImgClient::new(&getimg_api_key, &getimg_model),
        }
    }
}

impl Default for GeminiClient {
    /// 
    fn default() -> Self {
        let model = var("GEMINI_MODEL")
            .unwrap_or("gemini-pro".to_string())
            .to_owned();
        let api_key = var("GEMINI_API_KEY")
            .unwrap_or_default()
            .to_owned();

        Self {
            client: GemsClient::new(&api_key, &model),
        }
    }
}

impl GeminiClient {
    /// generate
    pub async fn generate(&self, prompt: String) -> Result<String> {
        let gemini_response = match self.client.clone()
            .generate_content(&prompt)
        .await {
            Ok(response) => strip_code_blocks(&response),
            Err(_err) => Default::default(),
        };

        Ok(gemini_response)
    }

    /// generate text from image
    pub async fn generate_text_from_image(&mut self, prompt: &str, image_path: &str) -> Result<String>{
        let base64_image_data = match load_and_encode_image(image_path) {
            Ok(data) => data,
            Err(_) => {
                debug!("[*] {:?}: Error loading image!", image_path);
                "".to_string()
            }
        };

        let response = self.client
            .generate_content_with_image(prompt, &base64_image_data)
            .await
            .unwrap();

        Ok(response)
    }

    pub async fn generate_image_from_text(&mut self, text_prompt: &str, negative_prompt: Option<&str>, img_path: &str, tasks: &Tasks) -> Result<()> {
        //let img_path = self.workspace.to_string() + "/img.jpg";

        // let text_prompt: String =
        //     format!("{}\n\nUser Prompt: {}", IMGGET_PROMPT, tasks.description);
        // let negative_prompt = Some("Disfigured, cartoon, blurry");

        // Generate image from text prompt
        let text_response = self
            .img_client
            .generate_image_from_text(
                &text_prompt,
                1024,
                1024,
                5,
                "jpeg",
                negative_prompt,
                Some(512),
            )
            .await?;

        // Save text response image to file
        save_image(&text_response.image, &img_path).unwrap();

        info!(
            "[*] Image saved at {}",
            //self.agent.position(),
            img_path
        );

        Ok(())
    }
}

