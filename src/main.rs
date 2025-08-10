use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use tracing::{info, error};
use tracing_subscriber::EnvFilter;
use tracing_subscriber;

mod engine;
mod networking;
mod data;
mod ai_interface;
mod browser_interface;

use engine::{TaskManager, Config};

#[derive(Parser)]
#[command(name = "flash")]
#[command(about = "🤖 Flash AI - Your Intelligent Web Scraping Assistant")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
    
    /// Configuration file path
    #[arg(short, long, default_value = "flash.toml")]
    config: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactive chat mode with Flash AI
    Chat {
        /// Initial message to Flash
        #[arg(short, long)]
        message: Option<String>,
    },
    
    /// Execute a natural language scraping command
    Execute {
        /// The scraping task in natural language
        task: String,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        
        /// Enable stealth mode
        #[arg(short, long)]
        stealth: bool,
    },
    
    /// Start the web dashboard
    Dashboard {
        /// Port for the web interface
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    
    /// Manage proxy settings
    Proxy(ProxyArgs),
    
    /// Show system status
    Status,
}

#[derive(Args)]
struct ProxyArgs {
    #[command(subcommand)]
    action: ProxyAction,
}

#[derive(Subcommand)]
enum ProxyAction {
    /// Add a new proxy
    Add {
        /// Proxy URL (http://user:pass@host:port)
        url: String,
    },
    /// List all proxies
    List,
    /// Test proxy connectivity
    Test {
        /// Proxy ID to test
        id: Option<u32>,
    },
    /// Remove a proxy
    Remove {
        /// Proxy ID to remove
        id: u32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(format!("flash_core={}", log_level)))
        .init();
    
    info!("🤖 Flash AI starting up...");
    
    // Load configuration
    let config = Config::load(&cli.config)?;
    
    // Initialize core systems
    let task_manager = TaskManager::new(config).await?;
    
    match cli.command {
        Some(Commands::Chat { message }) => {
            info!("Starting interactive chat mode");
            run_chat_mode(&task_manager, message).await?;
        },
        
        Some(Commands::Execute { task, output, stealth }) => {
            info!("Executing task: {}", task);
            execute_task(&task_manager, &task, output, stealth).await?;
        },
        
        Some(Commands::Dashboard { port }) => {
            info!("Starting web dashboard on port {}", port);
            start_dashboard(&task_manager, port).await?;
        },
        
        Some(Commands::Proxy(proxy_args)) => {
            handle_proxy_command(&task_manager, proxy_args).await?;
        },
        
        Some(Commands::Status) => {
            show_status(&task_manager).await?;
        },
        
        None => {
            // Default to chat mode
            info!("No command specified, starting chat mode");
            run_chat_mode(&task_manager, None).await?;
        }
    }
    
    Ok(())
}

async fn run_chat_mode(task_manager: &TaskManager, initial_message: Option<String>) -> Result<()> {
    println!("🤖 Flash AI Chat Mode");
    println!("=====================");
    println!("Type your scraping requests in natural language!");
    println!("Examples:");
    println!("  - Find 100 universities in Japan with contact info");
    println!("  - Get all restaurants in Paris with Michelin stars");
    println!("  - Scrape job postings for AI engineers in Silicon Valley");
    println!();
    
    if let Some(msg) = initial_message {
        println!("👤 You: {}", msg);
        let response = task_manager.process_natural_language(&msg).await?;
        println!("🤖 Flash: {}", response);
    }
    
    loop {
        print!("👤 You: ");
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() || input == "exit" || input == "quit" {
            println!("🤖 Flash: Goodbye! Happy scraping! 🚀");
            break;
        }
        
        match task_manager.process_natural_language(input).await {
            Ok(response) => println!("🤖 Flash: {}", response),
            Err(e) => {
                error!("Error processing request: {}", e);
                println!("🤖 Flash: Sorry, I encountered an error: {}", e);
            }
        }
    }
    
    Ok(())
}

async fn execute_task(
    task_manager: &TaskManager, 
    task: &str, 
    output: Option<String>, 
    stealth: bool
) -> Result<()> {
    let result = task_manager.execute_task(task, output, stealth).await?;
    println!("✅ Task completed successfully!");
    println!("📊 Results: {}", result);
    Ok(())
}

async fn start_dashboard(task_manager: &TaskManager, port: u16) -> Result<()> {
    println!("🌐 Starting Flash AI Dashboard on http://localhost:{}", port);
    // This will be implemented when we build the web interface
    task_manager.start_web_interface(port).await
}

async fn handle_proxy_command(task_manager: &TaskManager, args: ProxyArgs) -> Result<()> {
    match args.action {
        ProxyAction::Add { url } => {
            task_manager.add_proxy(&url).await?;
            println!("✅ Proxy added successfully");
        },
        ProxyAction::List => {
            let proxies = task_manager.list_proxies().await?;
            println!("📋 Configured Proxies:");
            for (id, proxy) in proxies {
                println!("  {} - {}", id, proxy);
            }
        },
        ProxyAction::Test { id } => {
            let result = task_manager.test_proxy(id).await?;
            println!("🧪 Proxy test result: {}", result);
        },
        ProxyAction::Remove { id } => {
            task_manager.remove_proxy(id).await?;
            println!("✅ Proxy removed successfully");
        },
    }
    Ok(())
}

async fn show_status(task_manager: &TaskManager) -> Result<()> {
    let status = task_manager.get_status().await?;
    println!("🤖 Flash AI System Status");
    println!("========================");
    println!("Status: {}", status.health);
    println!("Active Tasks: {}", status.active_tasks);
    println!("Proxies: {}", status.proxy_count);
    println!("Stealth Mode: {}", if status.stealth_active { "ON" } else { "OFF" });
    println!("Last Activity: {}", status.last_activity);
    Ok(())
}
