use mlua::{ExternalResult, UserData};
use twilight_model::{
    channel::Message,
    gateway::payload::incoming::{MessageDelete, MessageUpdate},
};

use std::sync::Arc;

#[derive(Clone)]
pub struct LuaOnMessageEvent(pub Message, pub Arc<twilight_http::Client>);

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

#[derive(Clone)]
pub struct LuaOnMessageDeleteEvent(pub MessageDelete, pub Arc<twilight_http::Client>);

impl UserData for LuaOnMessageDeleteEvent {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.0.id.get()));
        fields.add_field_method_get("channel_id", |_, this| Ok(this.0.channel_id.get()))
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

#[derive(Clone)]
pub struct LuaOnMessageUpdateEvent(pub MessageUpdate, pub Arc<twilight_http::Client>);

impl UserData for LuaOnMessageUpdateEvent {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("content", |_, this| Ok(this.0.content.clone()));
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
