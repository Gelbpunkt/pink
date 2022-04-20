use futures_util::StreamExt;
use mlua::{ChunkMode, Compiler, ExternalResult, Function, Lua, UserData};
use serde::Deserialize;
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt};
use twilight_gateway::{cluster::ShardScheme, Event, EventTypeFlags, Intents};
use twilight_model::channel::Message;

use std::{env, error::Error, fs::read_to_string, str::FromStr, sync::Arc};

const LUA_MESSAGE_HANDLER: &str = r#"
string.startswith = function(self, str)
    return self:find('^' .. str) ~= nil
end

function on_message(message)
    if message.content:startswith("!hello") then
        print("Lua got a message with this content:")
        print(message.content)
        print("Replying now!")
        message:reply("Hello?")
        print("Done replying!")
    end
end
"#;

#[derive(Clone)]
struct LuaOnMessageEvent(Message, Arc<twilight_http::Client>);

impl UserData for LuaOnMessageEvent {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("content", |_, this| Ok(this.0.content.clone()))
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("reply", |_, this, content: String| async move {
            this.1
                .create_message(this.0.channel_id)
                .content(&content)
                .to_lua_err()?
                .exec()
                .await
                .to_lua_err()?;

            Ok(())
        });
    }
}

#[derive(Deserialize)]
struct Config {
    token: String,
}

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let level_filter = LevelFilter::from_str(
        env::var("RUST_LOG")
            .unwrap_or_else(|_| String::from("INFO"))
            .as_str(),
    )
    .unwrap_or(LevelFilter::INFO);

    let fmt_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(level_filter)
        .init();

    tracing::info!("Loading configuration from config.json");

    let mut config_bytes = read_to_string("config.json")?;
    let config: Config = simd_json::from_str(&mut config_bytes)?;

    tracing::info!("Initializing Lua runtime and compiler");

    let lua = Lua::new();

    let globals = lua.globals();

    // TODO: Add logging global methods (log.info, etc.)

    let compiler = Compiler::new()
        .set_optimization_level(2)
        .set_debug_level(0)
        .set_coverage_level(0);

    // Luau allows modules via
    // env::set_var("LUAU_PATH", temp_dir.path().join("?.luau"));
    // local module = require("module")

    tracing::info!("Loading Lua code");

    let on_message_bytecode = compiler.compile(LUA_MESSAGE_HANDLER);

    lua.load(&on_message_bytecode)
        .set_mode(ChunkMode::Binary)
        .eval()?;

    let on_message = globals.get::<_, Function>("on_message")?;

    tracing::info!("Connecting to Discord");

    let http = Arc::new(twilight_http::Client::new(config.token.clone()));

    let (cluster, mut events) = twilight_gateway::Cluster::builder(
        config.token,
        Intents::GUILDS
            | Intents::MESSAGE_CONTENT
            | Intents::GUILD_MESSAGES
            | Intents::DIRECT_MESSAGES,
    )
    .shard_scheme(ShardScheme::Auto)
    .event_types(EventTypeFlags::MESSAGE_CREATE | EventTypeFlags::READY)
    .http_client(http.clone())
    .build()
    .await?;

    cluster.up().await;

    while let Some((_shard_id, event)) = events.next().await {
        tracing::debug!("Received {:?}", event);

        match event {
            Event::MessageCreate(evt) => {
                let lua_msg = LuaOnMessageEvent(evt.0, http.clone());
                on_message.call_async::<_, ()>(lua_msg).await?;
            }
            Event::Ready(_) => {
                tracing::info!("Bot is ready");
            }
            _ => {} // TODO: Add more event dispatchers
        }
    }

    tracing::info!("Event stream from Discord ended");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run())
}
