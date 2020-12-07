import { MessageReaction, User } from "discord.js";

import { Module, IModuleConfig } from "../structures/Module";

const { guild_access: { enabled, channel, message: messageID, emoji: emojiID, role } } = require("../../config.json");

class GuildAccessReaction extends Module
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

		const { message, emoji } = reaction;
		if (!message.guild) return;
		if (message.id !== messageID) return;
		if (message.channel.id !== channel) return;
		if ((emoji.id ?? emoji.name) !== emojiID) return;

		const member = await message.guild.members.fetch(user);
		if (member.roles.cache.has(role))
		{
			await member.roles.remove(role);
		}
		else
		{
			console.error(`User ${user.id} was missing the role to remove.`);
		}
	}
}

export { GuildAccessReaction as Module };
