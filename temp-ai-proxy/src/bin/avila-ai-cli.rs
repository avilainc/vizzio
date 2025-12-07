//! Cliente CLI para testar AI Proxy

use avila_ai_proxy::prelude::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "avila-ai")]
#[command(about = "Cliente CLI para Avila AI Proxy", long_about = None)]
struct Cli {
    /// URL do servidor AI Proxy
    #[arg(long, default_value = "http://localhost:8000")]
    url: String,

    /// API Key
    #[arg(long, env = "AVILA_API_KEY")]
    api_key: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Chat interativo
    Chat {
        /// Modelo a usar
        #[arg(long, default_value = "dolphin-mistral")]
        model: String,

        /// Mensagem
        message: String,
    },

    /// Completar código
    Complete {
        /// Arquivo de código
        #[arg(long)]
        file: String,

        /// Linguagem
        #[arg(long)]
        language: String,

        /// Posição do cursor
        #[arg(long, default_value = "0")]
        cursor: usize,
    },

    /// Criar API key (requer admin)
    CreateKey {
        /// Nome da key
        name: String,

        /// Tier (free/paid/admin)
        #[arg(long, default_value = "free")]
        tier: String,

        /// Dias até expirar
        #[arg(long, default_value = "365")]
        expires_days: i64,
    },

    /// Ver uso da API key
    Usage,

    /// Listar modelos
    Models,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    match cli.command {
        Commands::Chat { model, message } => {
            let response = client
                .post(format!("{}/v1/chat/completions", cli.url))
                .header("Authorization", format!("Bearer {}", cli.api_key))
                .json(&serde_json::json!({
                    "model": model,
                    "messages": [
                        {"role": "user", "content": message}
                    ],
                    "temperature": 0.7,
                    "max_tokens": 2000
                }))
                .send()
                .await?;

            if !response.status().is_success() {
                eprintln!("Erro: {}", response.text().await?);
                std::process::exit(1);
            }

            let result: serde_json::Value = response.json().await?;
            println!("{}", result["choices"][0]["message"]["content"].as_str().unwrap_or(""));
        }

        Commands::Complete {
            file,
            language,
            cursor,
        } => {
            let code = std::fs::read_to_string(&file)?;

            let response = client
                .post(format!("{}/v1/code/completions", cli.url))
                .header("Authorization", format!("Bearer {}", cli.api_key))
                .json(&serde_json::json!({
                    "code": code,
                    "language": language,
                    "cursor_position": cursor,
                    "model": "dolphin-mistral",
                    "max_tokens": 500
                }))
                .send()
                .await?;

            if !response.status().is_success() {
                eprintln!("Erro: {}", response.text().await?);
                std::process::exit(1);
            }

            let result: serde_json::Value = response.json().await?;
            println!("{}", result["completion"].as_str().unwrap_or(""));
        }

        Commands::CreateKey {
            name,
            tier,
            expires_days,
        } => {
            let response = client
                .post(format!("{}/v1/keys", cli.url))
                .header("Authorization", format!("Bearer {}", cli.api_key))
                .json(&serde_json::json!({
                    "name": name,
                    "tier": tier,
                    "expires_days": expires_days
                }))
                .send()
                .await?;

            if !response.status().is_success() {
                eprintln!("Erro: {}", response.text().await?);
                std::process::exit(1);
            }

            let result: serde_json::Value = response.json().await?;
            println!("✅ API Key criada:");
            println!("Key: {}", result["key"].as_str().unwrap_or(""));
            println!("Name: {}", result["name"].as_str().unwrap_or(""));
            println!("Tier: {}", result["tier"].as_str().unwrap_or(""));
        }

        Commands::Usage => {
            let response = client
                .get(format!("{}/v1/usage", cli.url))
                .header("Authorization", format!("Bearer {}", cli.api_key))
                .send()
                .await?;

            if !response.status().is_success() {
                eprintln!("Erro: {}", response.text().await?);
                std::process::exit(1);
            }

            let result: serde_json::Value = response.json().await?;
            println!("📊 Uso da API Key:");
            println!(
                "Nome: {}",
                result["key_info"]["name"].as_str().unwrap_or("")
            );
            println!(
                "Tier: {}",
                result["key_info"]["tier"].as_str().unwrap_or("")
            );
            println!(
                "Requests (último minuto): {}",
                result["usage"]["requests_last_minute"]
            );
            println!(
                "Requests (restantes): {}",
                result["usage"]["requests_remaining"]
            );
            println!(
                "Requests (total): {}",
                result["usage"]["requests_total"]
            );
        }

        Commands::Models => {
            let response = client
                .get(format!("{}/models", cli.url))
                .header("Authorization", format!("Bearer {}", cli.api_key))
                .send()
                .await?;

            if !response.status().is_success() {
                eprintln!("Erro: {}", response.text().await?);
                std::process::exit(1);
            }

            let result: serde_json::Value = response.json().await?;
            println!("🤖 Modelos disponíveis:");
            if let Some(models) = result["models"].as_object() {
                for (id, info) in models {
                    println!("\n  • {} ({})", id, info["provider"].as_str().unwrap_or(""));
                    println!("    {}", info["description"].as_str().unwrap_or(""));
                    println!("    Max tokens: {}", info["max_tokens"]);
                }
            }
        }
    }

    Ok(())
}
