//! Adaptador para integrar AI Proxy com Avila Copilot LSP
//!
//! Este módulo permite que o Avila Copilot LSP use o AI Proxy
//! em vez do modelo local, conectando com Ollama sem censura

use avila_ai_proxy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AiProxyAdapter {
    base_url: String,
    api_key: String,
    client: reqwest::Client,
}

impl AiProxyAdapter {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
            client: reqwest::Client::new(),
        }
    }

    /// Gera code completion usando AI Proxy
    pub async fn complete_code(
        &self,
        code: &str,
        language: &str,
        cursor_position: usize,
    ) -> Result<String> {
        let url = format!("{}/v1/code/completions", self.base_url);

        let request = serde_json::json!({
            "code": code,
            "language": language,
            "cursor_position": cursor_position,
            "model": "dolphin-mistral",
            "max_tokens": 500
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;

        if !response.status().is_success() {
            return Err(Error::Internal(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;

        Ok(result["completion"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }

    /// Chat para perguntas gerais
    pub async fn chat(
        &self,
        messages: Vec<(String, String)>, // Vec<(role, content)>
        model: Option<&str>,
    ) -> Result<String> {
        let url = format!("{}/v1/chat/completions", self.base_url);

        let messages: Vec<serde_json::Value> = messages
            .iter()
            .map(|(role, content)| {
                serde_json::json!({
                    "role": role,
                    "content": content
                })
            })
            .collect();

        let request = serde_json::json!({
            "model": model.unwrap_or("dolphin-mistral"),
            "messages": messages,
            "temperature": 0.7,
            "max_tokens": 2000
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;

        if !response.status().is_success() {
            return Err(Error::Internal(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;

        Ok(result["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }

    /// Detecta bugs no código
    pub async fn detect_bugs(&self, code: &str, language: &str) -> Result<Vec<String>> {
        let prompt = format!(
            "Analise o código {} abaixo e liste APENAS os bugs encontrados, um por linha:\n\n```{}\n{}\n```",
            language, language, code
        );

        let response = self
            .chat(vec![("user".to_string(), prompt)], Some("dolphin-mistral"))
            .await?;

        // Parse resposta em linhas
        Ok(response
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.trim().to_string())
            .collect())
    }

    /// Gera documentação
    pub async fn generate_docs(&self, code: &str, language: &str) -> Result<String> {
        let prompt = format!(
            "Gere documentação detalhada para o código {} abaixo:\n\n```{}\n{}\n```",
            language, language, code
        );

        self.chat(vec![("user".to_string(), prompt)], Some("dolphin-mistral"))
            .await
    }

    /// Gera testes
    pub async fn generate_tests(&self, code: &str, language: &str) -> Result<String> {
        let prompt = format!(
            "Gere testes unitários completos para o código {} abaixo:\n\n```{}\n{}\n```",
            language, language, code
        );

        self.chat(vec![("user".to_string(), prompt)], Some("dolphin-mistral"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requer servidor rodando
    async fn test_code_completion() {
        let adapter = AiProxyAdapter::new(
            "http://localhost:8000".to_string(),
            "avila_test_key".to_string(),
        );

        let result = adapter
            .complete_code("fn fibonacci(n: u32) ->", "rust", 25)
            .await;

        assert!(result.is_ok());
        println!("Completion: {}", result.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn test_chat() {
        let adapter = AiProxyAdapter::new(
            "http://localhost:8000".to_string(),
            "avila_test_key".to_string(),
        );

        let result = adapter
            .chat(
                vec![("user".to_string(), "Explique Rust em 50 palavras".to_string())],
                None,
            )
            .await;

        assert!(result.is_ok());
        println!("Response: {}", result.unwrap());
    }
}
