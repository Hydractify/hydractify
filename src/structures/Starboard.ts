import
{
	MessageEmbed,
	MessageReaction,
	TextChannel,
	User,
} from "discord.js";
import { getConnection, Repository, InsertResult } from "typeorm";

import StarboardEntity from "../entity/Starboard";

const { starboard: { emojis, channel, threshold } } = require("../../config.json");

export class Starboard
{
	private readonly repository: Repository<StarboardEntity>;

	public reaction: MessageReaction;

	public constructor(reaction: MessageReaction)
	{
		this.repository = getConnection().getRepository(StarboardEntity);
		this.reaction = reaction;
	}

	private create(stars: number): Promise<InsertResult>
	{
		return this.repository.insert({
			messageId: this.reaction.message.id,
			stars: stars,
		});
	}

	private getStarboard(): Promise<StarboardEntity | undefined>
	{
		return this.repository.findOne(this.reaction.message.id);
	}

	public async update(): Promise<number | void>
	{
		const message = await this.reaction.message.fetch();
		const stars = new Set();

		for (const [_, reac] of message.reactions.cache)
		{
			if (!emojis.includes(reac.emoji.toString())) continue;

			const users = await reac.users.fetch();

			users.forEach((user: User) =>
			{
				if (user.id !== message.author.id) stars.add(user.id);
			});
		}

		if (!await this.getStarboard())
		{
			await this.create(stars.size);

			return stars.size;
		}

		await this.repository.update(this.reaction.message.id, { stars: stars.size });

		return stars.size;
	}

	public async updateMessage(stars: number): Promise<void>
	{
		const starboard = await this.getStarboard();
		const message = await this.reaction.message.fetch();
		const starboardChannel = message.guild!.channels.resolve(channel) as TextChannel;

		if (!starboard) return;

		if (starboard.starboardId)
		{
			const starboardMsg = await starboardChannel.messages.fetch(starboard.starboardId);

			if (stars < threshold)
			{
				await starboardMsg!.delete();
				await this.repository.update(this.reaction.message.id, { starboardId: undefined });

				return;
			}

			await starboardMsg!.edit(`**${starboard.stars}**\\🌟『${message.channel}』`);
			return;
		}

		if (stars < threshold) return;

		const starboardEmbed = new MessageEmbed({
			author: {
				name: message.author.tag,
				iconURL: message.author.displayAvatarURL(),
			},
			color: 0xffcf05,
			description: `[Original](${message.url})`,
			image: {
				url: message.attachments.size ? message.attachments.first()!.url : undefined,
			},
		}).setTimestamp(message.createdTimestamp);

		if (message.content) starboardEmbed.addField("Message", message.content);
		if (message.embeds.length)
		{
			if (message.embeds[0].thumbnail) starboardEmbed.setImage(message.embeds[0].thumbnail.url);
		}
		if (message.attachments.size && !starboardEmbed.image)
		{
			starboardEmbed.setImage(message.attachments.first()!.url);
		}

		const msg = await starboardChannel.send(`**${stars}**\\🌟『${message.channel}』`, starboardEmbed);

		await this.repository.update(this.reaction.message.id, { starboardId: msg.id });
	}
}

export interface IStarboardConfig
{
	enabled: boolean;
	channel: string;
	threshold: number;
	emojis: string[];
}
