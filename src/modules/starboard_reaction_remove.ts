import { MessageReaction, User } from "discord.js";

import { Module, IModuleConfig } from "../structures/Module";
import { Starboard } from "../structures/Starboard";

const { starboard: { enabled } } = require("../../config.json");

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

		let { message } = reaction;
		message = await message.fetch();

		if (!message.guild) return;
		if (message.author.id === user.id) return;

		const starboard = new Starboard(reaction);

		const stars = await starboard.update();
		if (!stars) return;

		await starboard.updateMessage(stars);
	}
}

export { StarboardRemove as Module };
