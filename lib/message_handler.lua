function on_message(message)
    if message.content:startswith("!hello") then
        log.info("Lua got a message with this content:")
        log.info(message.content)
        log.info("Replying now!")
        message:reply("Hello?")
        log.info("Done replying!")
    end
end
