#[cfg(feature = "llm")]
use {
    super::{GitmojiMatcher, MatcherResult},
    crate::emoji::EmojiLookup,
    anyhow::{Result, anyhow},
    regex::Regex,
    reqwest::Client,
    serde::{Deserialize, Serialize},
    serde_json::Value,
    std::time::Duration,
};

#[cfg(feature = "llm")]
use std::env;

/// Request/Response structures for SiliconFlow API calls
#[cfg(feature = "llm")]
#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[cfg(feature = "llm")]
#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[cfg(feature = "llm")]
#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[cfg(feature = "llm")]
#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

/// Supported LLM providers (currently only SiliconFlow)
#[cfg(feature = "llm")]
#[derive(Debug, Clone, PartialEq)]
pub enum LLMProvider {
    SiliconFlow,
}

#[cfg(feature = "llm")]
impl LLMProvider {
    /// Get the base URL for the provider
    fn base_url(&self) -> &'static str {
        match self {
            LLMProvider::SiliconFlow => "https://api.siliconflow.cn/v1",
        }
    }

    /// Get the chat completions endpoint
    fn chat_endpoint(&self) -> String {
        format!("{}/chat/completions", self.base_url())
    }

    /// Execute API request for SiliconFlow
    async fn call_api(
        &self,
        client: &Client,
        api_key: &str,
        model: &str,
        prompt: &str,
    ) -> Result<String> {
        let request_body = ChatRequest {
            model: model.to_string(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let response = client
            .post(self.chat_endpoint())
            .header("Authorization", format!("Bearer {api_key}"))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "SiliconFlow API call failed with status {}: {}",
                status,
                error_text
            ));
        }

        let chat_response: ChatResponse = response.json().await?;

        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow!("No response from SiliconFlow provider"))
        }
    }
}

/// Supported LLM models for SiliconFlow
#[cfg(feature = "llm")]
#[derive(Debug, Clone, PartialEq)]
pub enum LLMModel {
    Qwen2_7bInstruct,
    Qwen25_7bInstruct,
}

#[cfg(feature = "llm")]
impl LLMModel {
    /// Get the model identifier string for API calls
    fn model_id(&self) -> &'static str {
        match self {
            LLMModel::Qwen2_7bInstruct => "Qwen/Qwen2-7B-Instruct",
            LLMModel::Qwen25_7bInstruct => "Qwen/Qwen2.5-7B-Instruct",
        }
    }

    /// Check if model is compatible with provider (all models are compatible with SiliconFlow)
    fn is_compatible_with(&self, provider: &LLMProvider) -> bool {
        match provider {
            LLMProvider::SiliconFlow => true,
        }
    }
}

/// LLM configuration
#[cfg(feature = "llm")]
#[derive(Debug, Clone)]
pub struct LLMConfig {
    pub provider: LLMProvider,
    pub api_key: String,
    pub model: LLMModel,
}

#[cfg(feature = "llm")]
impl LLMConfig {
    /// Create new LLM config with validation
    pub fn new(provider: LLMProvider, api_key: String, model: LLMModel) -> Result<Self> {
        if api_key.trim().is_empty() {
            return Err(anyhow!("API key cannot be empty"));
        }

        if !model.is_compatible_with(&provider) {
            return Err(anyhow!(
                "Model {:?} is not compatible with provider {:?}",
                model,
                provider
            ));
        }

        Ok(Self {
            provider,
            api_key,
            model,
        })
    }

    /// Create new LLM config loading API key from environment variables
    /// Tries to load from .env file first, then falls back to system environment
    pub fn from_env(provider: LLMProvider, model: LLMModel) -> Result<Self> {
        // Try to load .env file if it exists (ignore errors if file doesn't exist)
        #[cfg(feature = "llm")]
        let _ = dotenvy::dotenv();

        // Load API key from environment
        let api_key = env::var("API_KEY").map_err(|_| {
            anyhow!("API key not found in environment. Please set API_KEY environment variable")
        })?;

        Self::new(provider, api_key, model)
    }

    /// Check if API key is available in environment variables
    pub fn is_api_key_available() -> bool {
        // Try to load .env file if it exists (ignore errors if file doesn't exist)
        #[cfg(feature = "llm")]
        let _ = dotenvy::dotenv();

        env::var("API_KEY").is_ok()
    }
}

/// LLM Matcher implementation
#[cfg(feature = "llm")]
pub struct LLMMatcher {
    config: LLMConfig,
    client: Client,
    gitmoji_data: String,
}

#[cfg(feature = "llm")]
impl LLMMatcher {
    /// Create new LLM matcher with configuration
    pub fn new(config: LLMConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| Client::new());

        let gitmoji_data = Self::load_gitmoji_data();

        Self {
            config,
            client,
            gitmoji_data,
        }
    }

    /// Load gitmoji data for the prompt
    fn load_gitmoji_data() -> String {
        let json_content = include_str!("../../fixtures/gitmojis.json");

        // Parse and extract only code and description
        if let Ok(value) = serde_json::from_str::<Value>(json_content) {
            if let Some(gitmojis) = value["gitmojis"].as_array() {
                let simplified: Vec<_> = gitmojis
                    .iter()
                    .filter_map(|emoji| {
                        let code = emoji["code"].as_str()?;
                        let description = emoji["description"].as_str()?;
                        Some(format!("{code}: {description}"))
                    })
                    .collect();

                return simplified.join("\n");
            }
        }

        // Fallback if JSON parsing fails
        String::from(
            ":sparkles: Introduce new features.\n:bug: Fix a bug.\n:memo: Add or update documentation.",
        )
    }

    /// Generate the LLM prompt
    fn create_prompt(&self, commit_message: &str) -> String {
        format!(
            r#"You are a git commit message expert. Your task is to match a commit message to the most appropriate gitmoji and improve the commit message if needed.

Available gitmojis:
{gitmoji_data}

Instructions:
1. Analyze the commit message to understand what type of change it represents
2. Select the most appropriate gitmoji from the list above
3. Improve the commit message if it's unclear or poorly written
4. Respond ONLY with the final commit message in this exact format: ":emoji_code: improved_message"

Examples:
- Input: "feat: add new user auth"
  Output: ":sparkles: add new user authentication"
- Input: "fix login bug"
  Output: ":bug: fix login bug"
- Input: "docs update"
  Output: ":memo: update documentation"

User Commit Message:
{commit_message}"#,
            gitmoji_data = self.gitmoji_data
        )
    }

    /// Call the LLM API
    async fn call_llm_api(&self, prompt: &str) -> Result<String> {
        self.config
            .provider
            .call_api(
                &self.client,
                &self.config.api_key,
                self.config.model.model_id(),
                prompt,
            )
            .await
    }

    /// Parse LLM response to extract gitmoji and message
    fn parse_llm_response(&self, response: &str) -> Option<(String, String)> {
        // Pattern to match ":emoji_code: message" format
        let re = Regex::new(r"^:([a-zA-Z0-9_]+):\s*(.+)$").ok()?;

        if let Some(captures) = re.captures(response.trim()) {
            let emoji_code = format!(":{}:", captures.get(1)?.as_str());
            let message = captures.get(2)?.as_str().to_string();

            // Validate that the emoji code exists in our lookup
            if EmojiLookup::code_to_unicode(&emoji_code).is_some() {
                return Some((emoji_code, message));
            }
        }

        None
    }

    /// Match emoji with async LLM call
    pub async fn match_emoji_async(&self, message: &str) -> Result<MatcherResult> {
        if message.trim().is_empty() {
            return Ok(None);
        }

        let prompt = self.create_prompt(message);

        match self.call_llm_api(&prompt).await {
            Ok(response) => {
                if let Some((emoji_code, improved_message)) = self.parse_llm_response(&response) {
                    if EmojiLookup::code_to_unicode(&emoji_code).is_some() {
                        let formatted_message = format!("{emoji_code} {improved_message}");
                        return Ok(Some((emoji_code, formatted_message)));
                    }
                }

                // If parsing fails, return None to trigger fallback
                Ok(None)
            }
            Err(e) => {
                // If LLM call fails, return None to trigger fallback
                println!("LLM call failed: {e}");
                Ok(None)
            }
        }
    }
}

#[cfg(feature = "llm")]
impl GitmojiMatcher for LLMMatcher {
    fn match_emoji(&self, message: &str) -> Result<MatcherResult> {
        // Since the trait method is synchronous but LLM calls are async,
        // we need to use a runtime to block on the async call
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(self.match_emoji_async(message))
    }

    fn name(&self) -> &'static str {
        "llm"
    }
}

/// LLM Matcher with fallback to SimpleMatcher
#[cfg(feature = "llm")]
pub struct LLMWithFallbackMatcher {
    llm_matcher: LLMMatcher,
    fallback_matcher: super::simple::SimpleMatcher,
}

#[cfg(feature = "llm")]
impl LLMWithFallbackMatcher {
    /// Create new LLM matcher with fallback
    pub fn new(config: LLMConfig) -> Self {
        Self {
            llm_matcher: LLMMatcher::new(config),
            fallback_matcher: super::simple::SimpleMatcher::new(),
        }
    }

    /// Try LLM first, then fallback to simple matcher
    pub async fn match_emoji_async(&self, message: &str) -> Result<MatcherResult> {
        // Try LLM matcher first
        match self.llm_matcher.match_emoji_async(message).await? {
            Some(result) => Ok(Some(result)),
            None => {
                // Fallback to simple matcher
                println!("LLM matcher failed, falling back to simple matcher");
                self.fallback_matcher.match_emoji(message)
            }
        }
    }
}

#[cfg(feature = "llm")]
impl GitmojiMatcher for LLMWithFallbackMatcher {
    fn match_emoji(&self, message: &str) -> Result<MatcherResult> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(self.match_emoji_async(message))
    }

    fn name(&self) -> &'static str {
        "llm_with_fallback"
    }
}

#[cfg(all(test, feature = "llm"))]
mod tests {
    use super::*;

    #[test]
    fn test_llm_provider_urls() {
        assert_eq!(
            LLMProvider::SiliconFlow.base_url(),
            "https://api.siliconflow.cn/v1"
        );
    }

    #[test]
    fn test_llm_model_compatibility() {
        assert!(LLMModel::Qwen2_7bInstruct.is_compatible_with(&LLMProvider::SiliconFlow));
        assert!(LLMModel::Qwen25_7bInstruct.is_compatible_with(&LLMProvider::SiliconFlow));
    }

    #[test]
    fn test_llm_config_validation() {
        // Valid config
        let valid_config = LLMConfig::new(
            LLMProvider::SiliconFlow,
            "test-api-key".to_string(),
            LLMModel::Qwen25_7bInstruct,
        );
        assert!(valid_config.is_ok());

        // Empty API key should fail
        let invalid_config = LLMConfig::new(
            LLMProvider::SiliconFlow,
            "".to_string(),
            LLMModel::Qwen2_7bInstruct,
        );
        assert!(invalid_config.is_err());

        // Whitespace-only API key should fail
        let whitespace_config = LLMConfig::new(
            LLMProvider::SiliconFlow,
            "   ".to_string(),
            LLMModel::Qwen25_7bInstruct,
        );
        assert!(whitespace_config.is_err());
    }

    #[test]
    fn test_llm_matcher_creation() {
        let config = LLMConfig::new(
            LLMProvider::SiliconFlow,
            "test-api-key".to_string(),
            LLMModel::Qwen2_7bInstruct,
        )
        .unwrap();

        let matcher = LLMMatcher::new(config);
        assert_eq!(matcher.name(), "llm");
        assert!(!matcher.gitmoji_data.is_empty());
    }

    #[test]
    fn test_response_parsing() {
        let config = LLMConfig::new(
            LLMProvider::SiliconFlow,
            "test-key".to_string(),
            LLMModel::Qwen2_7bInstruct,
        )
        .unwrap();

        let matcher = LLMMatcher::new(config);

        // Valid response
        let result = matcher.parse_llm_response(":sparkles: add new feature");
        assert!(result.is_some());
        let (code, message) = result.unwrap();
        assert_eq!(code, ":sparkles:");
        assert_eq!(message, "add new feature");

        // Valid response with extra whitespace
        let result = matcher.parse_llm_response("  :bug: fix critical issue  ");
        assert!(result.is_some());
        let (code, message) = result.unwrap();
        assert_eq!(code, ":bug:");
        assert_eq!(message, "fix critical issue");

        // Invalid emoji code
        let result = matcher.parse_llm_response(":nonexistent: some message");
        assert!(result.is_none());

        // Invalid format
        let result = matcher.parse_llm_response("fix bug");
        assert!(result.is_none());

        // Invalid format with wrong emoji syntax
        let result = matcher.parse_llm_response("sparkles: add feature");
        assert!(result.is_none());
    }

    #[test]
    fn test_prompt_creation() {
        let config = LLMConfig::new(
            LLMProvider::SiliconFlow,
            "test-key".to_string(),
            LLMModel::Qwen2_7bInstruct,
        )
        .unwrap();

        let matcher = LLMMatcher::new(config);
        let prompt = matcher.create_prompt("fix login bug");

        assert!(prompt.contains("fix login bug"));
        assert!(prompt.contains(":sparkles:"));
        assert!(prompt.contains(":bug:"));
        assert!(prompt.contains("emoji_code"));
    }

    #[test]
    fn test_fallback_matcher_creation() {
        let config = LLMConfig::new(
            LLMProvider::SiliconFlow,
            "test-key".to_string(),
            LLMModel::Qwen2_7bInstruct,
        )
        .unwrap();

        let matcher = LLMWithFallbackMatcher::new(config);
        assert_eq!(matcher.name(), "llm_with_fallback");
    }

    #[test]
    fn test_gitmoji_data_loading() {
        let gitmoji_data = LLMMatcher::load_gitmoji_data();
        assert!(!gitmoji_data.is_empty());
        assert!(gitmoji_data.contains(":sparkles:"));
        assert!(gitmoji_data.contains(":bug:"));
        assert!(gitmoji_data.contains("new features"));
        assert!(gitmoji_data.contains("Fix a bug"));
    }
}
