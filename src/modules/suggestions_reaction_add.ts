import { MessageReaction, User } from "discord.js";

import { Module, IModuleConfig } from "../structures/Module";

const { suggestions: { enabled, upvoteEmoji, downvoteEmoji } } = require("../../config.json");
const emojis = [upvoteEmoji, downvoteEmoji];

class SuggestionsReactionAdd extends Module
{
	public constructor(config: IModuleConfig)
	{
		super({
			client: config.client,
			enabled,
			eventName: "messageReactionAdd",
		});
	}

	public async handle(reaction: MessageReaction, user: User): Promise<void>
	{
		if (user.bot) return;

		let { message } = reaction;
		message = await message.fetch();

		if (!message.guild) return;

		const emoji = emojis.find(v => v === reaction.emoji.toString());
		if (!emoji) return;

		if (message.author.id === user.id)
		{
			await reaction.users.remove(user);

			return;
		}


		for (const [_, reac] of message.reactions.cache)
		{
			if (reac.emoji.toString() !== emojis.find(v => v !== emoji)) continue;

			const users = await reac.users.fetch();

			users.forEach((u: User) =>
			{
				if (u.id === user.id) reac.users.remove(u);
			});
		}
	}
}

export { SuggestionsReactionAdd as Module };
