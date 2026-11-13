use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use crate::engine;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot { return; }
        let Some(body) = msg.content.strip_prefix("!rpp") else { return; };
        let query = body.trim();
        if query.is_empty() {
            let _ = msg.reply(&ctx.http,
                "Rust++ commands: `!rpp help`, `!rpp template cpp`, `!rpp explain rust`, `!rpp analyze <question>`").await;
            return;
        }
        if query.eq_ignore_ascii_case("help") {
            let _ = msg.reply(&ctx.http,
                "**Rust++ — built-in assistant**\n`!rpp template <language>`\n`!rpp explain <language>`\n`!rpp analyze <question>`\nSupported: C++, Rust, Python, JavaScript, Java, C#.\nNo AI model or AI API is used.").await;
            return;
        }
        let query = query.strip_prefix("analyze ").unwrap_or(query);
        let response = engine::answer(query).render_text();
        // Discord message limit is 2,000 characters. Keep output bounded.
        let response: String = response.chars().take(1850).collect();
        let _ = msg.reply(&ctx.http, response).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Rust++ connected as {}. Rule engine is local.", ready.user.name);
    }
}
