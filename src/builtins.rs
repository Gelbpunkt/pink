use mlua::{Lua, Result};

macro_rules! log_level_impl {
    ($log_level:ident, $lua:ident, $exports:ident) => {
        fn $log_level(lua: &Lua, text: String) -> Result<()> {
            // Get the calling stack's name
            if let Some(stack_item) = lua.inspect_stack(1) {
                let source = stack_item.source();

                if let Some(func_name) = source.source.and_then(|txt| std::str::from_utf8(txt).ok()) {
                    tracing::$log_level!(target: "pink::log", "{text} ({func_name})");
                }
            }

            Ok(())
        }

        $exports.set(stringify!($log_level), $lua.create_function($log_level)?)?;
    };
}

pub fn load(lua: &Lua) -> Result<()> {
    let exports = lua.create_table()?;

    log_level_impl!(error, lua, exports);
    log_level_impl!(warn, lua, exports);
    log_level_impl!(info, lua, exports);
    log_level_impl!(debug, lua, exports);
    log_level_impl!(trace, lua, exports);

    let globals = lua.globals();
    globals.set("log", exports)?;

    Ok(())
}
