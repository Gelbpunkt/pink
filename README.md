# pink

Pink is a mini experiment by myself to see how well LUA scripting can be integrated with Tokio to write simple chatbots.

## Why?

Most people who want a Discord bot can't actually code, but Lua is so simple that most of them should be able to write basic commands with it.

All that a developer has to do is provide the right framework - which in this case will be a multi-threaded tokio runtime driving twilight and sqlx.

## Todo

- [ ] Logging interface
- [ ] Decent standard library for all events
- [ ] Database interface
- [ ] Seperate out Lua code into files (something like /lib or /bot with all the modules)
- [ ] Reconsider sandboxed mode as a config option
