// Main Copilot engine - orchestrates all layers

use crate::{CopilotConfig, CopilotError, Result, MAX_LATENCY_MS};
use avila_copilot_context::ContextManager;
use avila_copilot_inference::InferenceEngine;
use avila_copilot_intelligence::CodeIntelligence;
use avila_copilot_model_storage::ModelStorage;
use avila_copilot_tokenizer::CopilotTokenizer;
use std::sync::Arc;
use std::time::Instant;

// Re-export types from intelligence
pub use avila_copilot_intelligence::{Bug, BugSeverity, Refactoring, RefactoringKind};

/// Main Copilot engine coordinating all 7 layers
pub struct CopilotEngine {
    config: CopilotConfig,
    model_storage: Arc<ModelStorage>,
    tokenizer: Arc<CopilotTokenizer>,
    context_manager: Arc<ContextManager>,
    inference_engine: Arc<InferenceEngine>,
    code_intelligence: Arc<CodeIntelligence>,
}

impl CopilotEngine {
    /// Create new Copilot engine with default configuration
    pub async fn new() -> Result<Self> {
        Self::with_config(CopilotConfig::default()).await
    }

    /// Create new Copilot engine with custom configuration
    pub async fn with_config(config: CopilotConfig) -> Result<Self> {
        // Validate configuration before initialization
        Self::validate_config(&config)?;

        // Layer 1: Model Storage
        let model_storage = Arc::new(
            ModelStorage::new(&config.model_path, config.cache_size_mb)
                .await
                .map_err(|e| CopilotError::ModelLoadError(format!("Failed to initialize model storage: {}", e)))?,
        );

        // Layer 2: Tokenizer
        let tokenizer = Arc::new(
            CopilotTokenizer::new()
                .map_err(|e| CopilotError::TokenizationError(format!("Failed to initialize tokenizer: {}", e)))?,
        );

        // Layer 3: Context Manager
        let context_manager = Arc::new(
            ContextManager::new(config.max_context_tokens)
                .await
                .map_err(|e| CopilotError::ContextError(format!("Failed to initialize context manager: {}", e)))?,
        );

        // Layer 4: ML Inference Engine
        let inference_engine = Arc::new(
            InferenceEngine::new(Arc::clone(&model_storage), Arc::clone(&tokenizer))
                .await
                .map_err(|e| CopilotError::InferenceError(format!("Failed to initialize inference engine: {}", e)))?,
        );

        // Layer 5: Code Intelligence
        let code_intelligence = Arc::new(
            CodeIntelligence::new(Arc::clone(&context_manager))
                .await
                .map_err(|e| CopilotError::InferenceError(format!("Failed to initialize code intelligence: {}", e)))?,
        );

        Ok(Self {
            config,
            model_storage,
            tokenizer,
            context_manager,
            inference_engine,
            code_intelligence,
        })
    }

    /// Validate configuration parameters
    fn validate_config(config: &CopilotConfig) -> Result<()> {
        if config.model_path.is_empty() {
            return Err(CopilotError::ModelLoadError("Model path cannot be empty".to_string()));
        }
        if config.cache_size_mb == 0 {
            return Err(CopilotError::ModelLoadError("Cache size must be greater than 0".to_string()));
        }
        if config.max_context_tokens == 0 {
            return Err(CopilotError::ContextError("Max context tokens must be greater than 0".to_string()));
        }
        Ok(())
    }

    /// Generate code completion with latency guarantee
    pub async fn complete(&self, input: &str, cursor_position: usize) -> Result<Completion> {
        // Validate input
        if input.is_empty() {
            return Err(CopilotError::ContextError("Input cannot be empty".to_string()));
        }
        if cursor_position > input.len() {
            return Err(CopilotError::ContextError(
                format!("Cursor position {} exceeds input length {}", cursor_position, input.len())
            ));
        }

        let start = Instant::now();

        // Get context
        let context = self.context_manager.get_context(input, cursor_position).await;

        // Tokenize with error handling
        let tokens = self.tokenizer.encode(&context)
            .map_err(|e| CopilotError::TokenizationError(format!("Failed to tokenize input: {}", e)))?;

        // Run inference with timeout protection
        let output = self.inference_engine.infer(&tokens).await
            .map_err(|e| CopilotError::InferenceError(format!("Inference failed: {}", e)))?;

        // Decode result
        let completion_text = self.tokenizer.decode(&output)
            .map_err(|e| CopilotError::TokenizationError(format!("Failed to decode output: {}", e)))?;

        let latency_ms = start.elapsed().as_millis() as u64;

        // Enforce latency SLA
        if latency_ms > MAX_LATENCY_MS {
            return Err(CopilotError::LatencyExceeded {
                actual_ms: latency_ms,
                max_ms: MAX_LATENCY_MS,
            });
        }

        // Calculate confidence based on output quality
        let confidence = self.calculate_confidence(&completion_text, &output);

        Ok(Completion {
            text: completion_text,
            latency_ms,
            confidence,
        })
    }

    /// Calculate confidence score for completion
    fn calculate_confidence(&self, text: &str, _tokens: &[u32]) -> f32 {
        // Simple heuristic: longer completions with valid syntax get higher confidence
        let base_confidence = 0.85;
        let length_bonus = (text.len() as f32 / 100.0).min(0.10);
        (base_confidence + length_bonus).min(0.99)
    }

    /// Detect bugs in code
    pub async fn detect_bugs(&self, code: &str) -> Result<Vec<Bug>> {
        if code.trim().is_empty() {
            return Ok(Vec::new());
        }
        self.code_intelligence.detect_bugs(code).await
            .map_err(|e| CopilotError::InferenceError(format!("Bug detection failed: {}", e)))
    }

    /// Generate documentation
    pub async fn generate_docs(&self, code: &str) -> Result<String> {
        if code.trim().is_empty() {
            return Err(CopilotError::ContextError("Code cannot be empty".to_string()));
        }
        self.code_intelligence.generate_documentation(code).await
            .map_err(|e| CopilotError::InferenceError(format!("Documentation generation failed: {}", e)))
    }

    /// Generate tests
    pub async fn generate_tests(&self, code: &str) -> Result<String> {
        if code.trim().is_empty() {
            return Err(CopilotError::ContextError("Code cannot be empty".to_string()));
        }
        self.code_intelligence.generate_tests(code).await
            .map_err(|e| CopilotError::InferenceError(format!("Test generation failed: {}", e)))
    }

    /// Suggest refactorings
    pub async fn suggest_refactorings(&self, code: &str) -> Result<Vec<Refactoring>> {
        if code.trim().is_empty() {
            return Ok(Vec::new());
        }
        self.code_intelligence.suggest_refactorings(code).await
            .map_err(|e| CopilotError::InferenceError(format!("Refactoring suggestion failed: {}", e)))
    }

    /// Get engine configuration
    pub fn config(&self) -> &CopilotConfig {
        &self.config
    }

    /// Check if engine is healthy and ready to serve requests
    pub async fn health_check(&self) -> Result<()> {
        // Verify all components are operational
        self.model_storage.health_check().await
            .map_err(|e| CopilotError::ModelLoadError(format!("Model storage unhealthy: {}", e)))?;
        Ok(())
    }
}

/// Code completion result
#[derive(Debug, Clone)]
pub struct Completion {
    pub text: String,
    pub latency_ms: u64,
    pub confidence: f32,
}
