function on_message(message)
    if message.content:startswith("!hello") then
        log.info("Lua got a message with this content:")
        log.info(message.content)
        log.info("Replying now!")
        message:reply("Hello?")
        log.info("Done replying!")
    end
end

function on_message_delete(message)
	log.info("Message deleted with id:")
	log.info(message.message_id)
	message:reply("Hey! I saw that!")
end

function on_message_update(message)
    log.info("Message was edited with content:")
    log.info(message.content)
    message:reply("What you tryna hide?")
end