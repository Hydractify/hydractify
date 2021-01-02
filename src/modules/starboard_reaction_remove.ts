import { MessageReaction, User, TextChannel } from "discord.js";
import { getConnection } from "typeorm";

import { Module, IModuleConfig } from "../structures/Module";
import StarboardEntity from "../entity/Starboard";

const { starboard: { enabled, channel, threshold, emojis } } = require("../../config.json");

class StarboardRemove extends Module
{
	public constructor(config: IModuleConfig)
	{
		super({
			client: config.client,
			enabled,
			eventName: "messageReactionRemove",
		});
	}

	public async handle(reaction: MessageReaction, user: User): Promise<void>
	{
		if (user.bot) return;

		const { message, emoji } = reaction;
		if (!message.guild) return;
		if (!emojis.includes(emoji.toString())) return;

		const repo = getConnection().getRepository(StarboardEntity);
		let starboard = await repo.findOne(message.id);
		starboard = await repo.save({
			messageId: message.id,
			starboardId: starboard ? starboard.starboardId : undefined,
			stars: starboard ? starboard.stars - 1 : 0,
		});

		const starboardChannel = message.guild.channels.resolve(channel) as TextChannel;

		if (starboard.stars < threshold)
		{
			starboardChannel.messages.resolve(starboard.starboardId!)!.delete();
			return;
		}

		if (starboard.starboardId)
		{
			starboardChannel.messages.resolve(starboard.starboardId)!
				.edit(`**${starboard.stars}**\\🌟『${message.channel}』`);
		}
	}
}

export { StarboardRemove as Module };
