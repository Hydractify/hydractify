import { Message } from "discord.js";

import { Module, IModuleConfig } from "../structures/Module";

const { suggestions: { channel: suggestionChannel, enabled, upvoteEmoji, downvoteEmoji } } = require("../../config.json");

class Suggestions extends Module
{
	public constructor(config: IModuleConfig)
	{
		super({
			client: config.client,
			enabled,
			eventName: "message",
		});
	}

	public async handle(message: Message): Promise<void>
	{
		if (message.author.bot) return;
		if (message.channel.type === "dm") return;
		if (message.channel === suggestionChannel) return;

		await message.react(upvoteEmoji);
		await message.react(downvoteEmoji);
	}
}

export { Suggestions as Module };
