use anyhow::{bail, Context, Result};
use std::{env, path::PathBuf};
use rust_plus_plus::{discord::Handler, engine, knowledge, parse_language, transform, validate};
use serenity::{Client, model::gateway::GatewayIntents};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--discord") {
        return run_discord().await;
    }
    match args.first().map(String::as_str).unwrap_or("help") {
        "template" if args.len() >= 2 => {
            let lang = parse_language(&args[1])?;
            println!("{}", knowledge::template(lang).render_text());
        }
        "explain" if args.len() >= 2 => {
            let lang = parse_language(&args[1])?;
            println!("{}", knowledge::explain_language(lang).render_text());
        }
        "analyze" if args.len() >= 3 => {
            println!("{}", engine::answer(&args[2..].join(" ")).render_text());
        }
        "transform" if args.len() >= 4 => {
            let lang = parse_language(&args[1])?;
            if args[3] != "add-header" { bail!("Supported transform: add-header"); }
            let path = PathBuf::from(&args[2]);
            let reply = transform::transform_file(&path, lang, "Generated/edited with Rust++")?;
            println!("{}", reply.render_text());
        }
        "check" if args.len() >= 3 => {
            let lang = parse_language(&args[1])?;
            let source = std::fs::read_to_string(&args[2]).context("Could not read source file")?;
            println!("{}", validate::lightweight_check(&source, lang).render_text());
        }
        _ => {
            println!("Rust++ — deterministic offline coding assistant\n\nCommands:\n  template <language>\n  explain <language>\n  analyze <question>\n  transform <language> <file> add-header\n  check <language> <file>\n  --discord\n\nLanguages: cpp, rust, python, javascript, java, csharp");
        }
    }
    Ok(())
}

async fn run_discord() -> Result<()> {
    let token = env::var("DISCORD_TOKEN")
        .context("Set DISCORD_TOKEN in your environment (or load it from your deployment secret manager)")?;
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .context("Could not create Discord client")?;
    client.start().await.context("Discord client stopped")?;
    Ok(())
}
