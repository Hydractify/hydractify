import { Message, MessageAttachment, TextChannel } from "discord.js";
import { getConnection, MoreThanOrEqual } from "typeorm";
import { schedule as scheduleCron } from "node-cron";

import MessageEntity from "../entity/Message";
import UserEntity from "../entity/User";
import { Module, IModuleConfig } from "../structures/Module";

const { story: { channel: channelId, enabled, eulaChannel } } = require("../../config.json");


class OneWordStory extends Module
{
	public constructor(config: IModuleConfig)
	{
		super({
			client: config.client,
			enabled,
			eventName: "message",
		});

		if (enabled)
		{
			// At 00:00 every Sunday
			scheduleCron("0 0 * * 0", this.compileStory.bind(this), { timezone: "Etc/UTC" });
		}
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
			await message.author.send("You have to wait for somebody else to send a word for the story!");

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

	private async compileStory(): Promise<void>
	{
		const trigger = new Date();
		// Ensuring our date is actually at midnight.
		trigger.setUTCHours(0, 0, 0, 0);
		// If it's Monday already, go back to Sunday
		trigger.setUTCDate(trigger.getUTCDate() - trigger.getUTCDay());

		const weekAgo = new Date(trigger.valueOf() - 1000 * 60 * 60 * 24 * 7);

		const messageRepo = getConnection().getRepository(MessageEntity);
		const messages = await messageRepo.find({
			order: {
				createdAt: "ASC",
			},
			where: {
				createdAt: MoreThanOrEqual(weekAgo),
			},
		});

		if (!messages.length) return;

		const content = messages.map(message => message.content).join(" ");

		const message = await (this.client.channels.resolve(channelId) as TextChannel).send(
			`Compiled story from ${this.formatDate(weekAgo)} until ${this.formatDate(trigger)}.`,
			new MessageAttachment(Buffer.from(content, "utf8"), "story.txt"),
		);

		// TODO: handle max pins
		await message.pin();
	}

	private formatDate(date: Date): string
	{
		return [
			date.getUTCFullYear().toString().padStart(4, "0"),
			(date.getUTCMonth() + 1).toString().padStart(2, "0"),
			date.getUTCDate().toString().padStart(2, "0"),
		].join("-");
	}
}

export { OneWordStory as Module };
