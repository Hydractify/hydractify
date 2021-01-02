import { MessageReaction, MessageEmbed, User, TextChannel } from "discord.js";
import { getConnection } from "typeorm";

import { Module, IModuleConfig } from "../structures/Module";
import StarboardEntity from "../entity/Starboard";

const { starboard: { enabled, channel, threshold, emojis } } = require("../../config.json");

class StarboardAdd extends Module
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

		console.log("?");
		const { message, emoji } = reaction;
		console.log("??");
		if (!message.guild) return;
		console.log("???");
		if (!emojis.includes(emoji.toString())) return;

		const repo = getConnection().getRepository(StarboardEntity);
		let starboard = await repo.findOne(message.id);
		starboard = await repo.save({
			messageId: message.id,
			starboardId: starboard ? starboard.starboardId : undefined,
			stars: starboard ? starboard.stars + 1 : 1,
		});
		if (starboard.stars < threshold) return;

		const starboardChannel = message.guild.channels.resolve(channel) as TextChannel;

		if (starboard.starboardId)
		{
			starboardChannel.messages.resolve(starboard.starboardId)!
				.edit(`**${starboard.stars}**\\🌟『${message.channel}』`);
		}
		else
		{
			const starboardEmbed = new MessageEmbed({
				author: {
					name: message.author.tag,
					iconURL: message.author.displayAvatarURL(),
				},
				color: 0xffcf05,
				description: `[Original](${message.url})`,
				image: {
					url: message.attachments.first() ? message.attachments.first()!.url : undefined,
				},
			}).setTimestamp();

			if (message.content) starboardEmbed.addField("Message", message.content);
			if (message.embeds.length)
			{
				if (message.embeds[0].image) starboardEmbed.setImage(message.embeds[0].image.url);
			}
			if (message.attachments.size && !starboardEmbed.image)
			{
				starboardEmbed.setImage(message.attachments.first()!.url);
			}

			const msg = await starboardChannel.send(`**${starboard.stars}**\\🌟『${message.channel}』`, starboardEmbed);
			repo.update(message.id, { starboardId: msg.id });
		}
	}
}

export { StarboardAdd as Module };
