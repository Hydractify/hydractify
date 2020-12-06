import { Message } from "discord.js";
import { getConnection } from "typeorm";

import MessageEntity from "../entity/Message";
import UserEntity from "../entity/User";
import { Module, IModuleConfig } from "../structures/Module";

const { channel: channelId, enabled, eulaChannel } = require("../../config/story.json");

export class StoryModule extends Module
{
	public constructor(config: IModuleConfig)
	{
		super({
			client: config.client,
			enabled,
			eventName: "message",
		});

		this.handle = this.handle.bind(this);
	}

	public async handle(message: Message): Promise<void>
	{
		if (message.author.bot) return;

		if (message.channel.type === "dm") return this.handleEula(message);

		if (channelId !== message.channel.id) return;

		const userRepo = getConnection().getRepository(UserEntity);
		const user = await userRepo.findOne(message.author.id);

		if (!user || !user.storyEula)
		{
			await message.delete();
			await message.author.send([
				"You first have to accept the EULA before sending a message.",
				`This discord application collects the words that you write in the <#${channelId}> and stores them in our server to then send a "story" in the end of every week.`,
				`If you accept this EULA and want to have your data deleted, you can do so by asking us in the <#${eulaChannel}> channel.`,
				"Lastly, to accept the EULA just send `accept` in this DM.",
			].join("\n"));

			return;
		}

		const messageRepo = getConnection().getRepository(MessageEntity);

		const lastMessage = await messageRepo.createQueryBuilder().orderBy("id", "DESC").getOne();

		if (lastMessage && lastMessage.authorId === message.author.id)
		{
			await message.delete();
			await message.author.send("You already sent a message for the story!");

			return;
		}

		if (/\s/.test(message.content))
		{
			await message.delete();
			await message.author.send("You have to send a single word!");

			return;
		}

		messageRepo.save({ id: message.id, authorId: message.author.id, content: message.content, createdAt: message.createdAt } as MessageEntity);
	}

	async handleEula(message: Message): Promise<void>
	{
		if (!/^accept/i.test(message.content)) return;

		const userRepo = getConnection().getRepository(UserEntity);
		await userRepo.save({ id: message.author.id, storyEula: new Date() } as UserEntity);

		await message.reply("You have successfully accepted the EULA!");
	}
}

export { StoryModule as Module };
