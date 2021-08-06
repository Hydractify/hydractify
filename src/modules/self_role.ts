import
{
	ApplicationCommandData,
	MessageComponentInteraction,
	CommandInteraction,
	CommandInteractionOption,
	Constants,
	DiscordAPIError,
	GuildMember,
	Interaction,
	MessageActionRowOptions,
	MessageButtonOptions,
	MessageOptions,
	Role,
	TextChannel,
	Snowflake,
} from "discord.js";
import { getConnection } from "typeorm";

import SelfRole from "../entity/SelfRole";
import { IModuleConfig, Module } from "../structures/Module";
import { chunkArray } from "../util";

const { ApplicationCommandOptionTypes, MessageButtonStyles, MessageComponentTypes } = Constants;

const { self_role: { enabled, channel: channelID } } = require("../../config.json");

const CUSTOM_ID_PREFIX = "self-role-";

enum SubCommand
// eslint-disable-next-line @typescript-eslint/indent
{
	add,
	remove,
	show,
	deploy,
	cleanup
}

export const SelfRoleCommand: ApplicationCommandData = {
	name: "selfrole",
	description: "Manage available self roles.",
	defaultPermission: false,
	options: [
		{
			name: SubCommand[SubCommand.add],
			type: ApplicationCommandOptionTypes.SUB_COMMAND,
			description: "Add a role to the list of available self roles.",
			options: [{
				name: "role",
				type: ApplicationCommandOptionTypes.ROLE,
				description: "The role to add to the list of self roles.",
				required: true,
			},
			{
				name: "style",
				type: ApplicationCommandOptionTypes.INTEGER,
				description: "The button style to use",
				choices: [
					{
						name: "Primary (Blurple)",
						value: MessageButtonStyles.PRIMARY,
					},
					{
						name: "Secondary (Grey)",
						value: MessageButtonStyles.SECONDARY,
					},
					{
						name: "Success (Green)",
						value: MessageButtonStyles.SUCCESS,
					},
					{
						name: "Danger (Red)",
						value: MessageButtonStyles.DANGER,
					},
				],
			},
			{
				name: "emoji",
				type: ApplicationCommandOptionTypes.STRING,
				description: "The emoji to use. (Make sure its actually valid)",
			}],
		},
		{
			name: SubCommand[SubCommand.remove],
			type: ApplicationCommandOptionTypes.SUB_COMMAND,
			description: "Remove a role from the list of available self roles.",
			options: [{
				name: "role",
				type: ApplicationCommandOptionTypes.ROLE,
				description: "The role to remove from the list of self roles.",
				required: true,
			}],
		},
		{
			name: SubCommand[SubCommand.show],
			type: ApplicationCommandOptionTypes.SUB_COMMAND,
			description: "Show all available self roles.",
		},
		{
			name: SubCommand[SubCommand.deploy],
			type: ApplicationCommandOptionTypes.SUB_COMMAND,
			description: "Deploys the self role message.",
			options: [{
				name: "message",
				type: ApplicationCommandOptionTypes.STRING,
				description: "The id of the message in the self roles channel to update.",
			}],
		},
		{
			name: SubCommand[SubCommand.cleanup],
			type: ApplicationCommandOptionTypes.SUB_COMMAND,
			description: "Removes deleted roles from the list of self assignable roles.",
		},
	],
};

class SelfRoleModule extends Module
{
	public constructor(config: IModuleConfig)
	{
		super({
			client: config.client,
			enabled,
			eventName: "interaction",
		});
	}

	protected override async handle(interaction: Interaction): Promise<void>
	{
		if (!interaction.guildID) return;
		if (interaction instanceof MessageComponentInteraction)
		{
			if (!interaction.customID.startsWith(CUSTOM_ID_PREFIX))
			{
				return;
			}

			return this.button(interaction);
		}

		if (interaction instanceof CommandInteraction)
		{
			if (interaction.commandName !== SelfRoleCommand.name)
			{
				return;
			}

			if (!interaction.guild)
			{
				await interaction.reply(
					"This guild is not available, the bot might not be fully started yet."
					+ " If this error persists, contact an administrator.",
					{ ephemeral: true },
				);

				return;
			}

			const subCommand = interaction.options.values().next().value as CommandInteractionOption;
			if (!subCommand) throw new Error("No sub command present.");

			const subCommandName = subCommand.name;

			if (isSubCommand(subCommandName))
			{
				this[subCommandName](interaction, subCommand.options!);
			}
		}
	}

	private async button(interaction: MessageComponentInteraction)
	{
		if (interaction.channelID !== channelID) return;
		if (!interaction.guild)
		{
			await interaction.reply(
				"This guild is not available, the bot might not be fully started yet.\n"
				+ "If this error persists, contact an administrator.",
				{ ephemeral: true },
			);

			return;
		}

		const roleID = interaction.customID.slice(CUSTOM_ID_PREFIX.length);

		const role = interaction.guild.roles.resolve(roleID);
		if (!role)
		{
			await interaction.reply(
				"Couldn't find the role for this button.\n"
				+ "If this error, persists contact an administrator.",
				{ ephemeral: true },
			);
			return;
		}

		const selfRole = await getConnection().getRepository(SelfRole).findOne(roleID);
		if (!selfRole)
		{
			await interaction.reply(
				`${role} is not designated to be self assignable.\n`
				+ "If this error, persists contact an administrator.",
				{ ephemeral: true },
			);

			return;
		}

		const member = interaction.member as GuildMember;
		if (member.roles.cache.has(role.id))
		{
			await member.roles.remove(role);
			await interaction.reply(`Successfully removed ${role} from you.`, { ephemeral: true });
		}
		else
		{
			await member.roles.add(role);
			await interaction.reply(`Successfully added ${role} to you.`, { ephemeral: true });
		}
	}

	private async add(interaction: CommandInteraction, options: NonNullable<CommandInteractionOption["options"]>)
	{
		const repo = getConnection().getRepository(SelfRole);

		const roleID = options.get("role")!.role!.id;
		const style = options.get("style")?.value as number ?? MessageButtonStyles.PRIMARY;
		let emoji = options.get("emoji")?.value as string ?? null;

		if (roleID === interaction.guildID)
		{
			await interaction.reply(
				"The everyone role may not be self-assignable; It's the everyone role after all.",
				{ ephemeral: true },
			);
			return;
		}

		// Try to detect custom emojis
		const match = emoji?.match(/^<(a)?:(\w{2,32}):(\d{16,21})>$/);
		if (match)
		{
			emoji = match.slice(1).join("|");
		}

		const selfrole = await repo.findOne(roleID);

		await repo.save({ id: roleID, style, emoji });
		const response = selfrole
			? `The <@&${roleID}> role is now self assignable.\n`
			+ "Make sure to `/selfrole deploy` this change."
			: `Updated style and emoji for the <@&${roleID}> role.\n`
			+ "Make sure to `/selfrole deploy` this change.";

		await interaction.reply(response);
	}

	private async remove(interaction: CommandInteraction, options: NonNullable<CommandInteractionOption["options"]>)
	{
		const repo = getConnection().getRepository(SelfRole);

		const roleID = options.get("role")!.role!.id;
		const selfrole = await repo.findOne(roleID);

		if (selfrole)
		{
			await repo.remove(selfrole);
			await interaction.reply(
				`The <@&${roleID}> role is no longer self assignable.\n`
				+ "Make sure to `/selfrole deploy` this change.",
			);
		}
		else
		{
			await interaction.reply(`The <@&${roleID}> role is not self assignable.`, { ephemeral: true });
		}
	}

	private async show(interaction: CommandInteraction)
	{
		const repo = getConnection().getRepository(SelfRole);

		const roles = await repo.find();

		if (!roles.length)
		{
			await interaction.reply("There are no self assignable roles.");
		}
		else
		{
			const roleString = roles.map(r =>
				`<@&${r.id}> (emoji: ${r.emoji ?? "none"}, style: ${MessageButtonStyles[r.style] ?? r.style})`,
			).join(", ");

			await interaction.reply(
				`Self assignable roles are: ${roleString}`,
			);
		}
	}

	private async deploy(interaction: CommandInteraction, options: NonNullable<CommandInteractionOption["options"]>)
	{
		const repo = getConnection().getRepository(SelfRole);

		const messageID = options.get("message")?.value as Snowflake;

		const channel = this.client.channels.resolve(channelID);
		if (!channel)
		{
			await interaction.reply(
				`The designated channel for self roles (<#${channelID}> / ${channelID}) could not be found.`,
			);

			return;
		}

		if (!(channel instanceof TextChannel))
		{
			await interaction.reply(
				`The designated channel for self roles (<#${channelID}> / ${channelID}) is not a text channel.`,
			);

			return;
		}

		const roles = (await repo.find())
			.map(r => [r, interaction.guild?.roles.resolve(r.id)])
			// eslint-disable-next-line @typescript-eslint/no-unused-vars
			.filter(([_, r]) => r) as [SelfRole, Role][];

		const messageOptions: MessageOptions = {
			content: "Click the buttons below to toggle a role on you.",
			components: chunkArray(roles, 5)
				.map<MessageActionRowOptions>(row =>
				({
					type: MessageComponentTypes.ACTION_ROW,
					components: row.map<MessageButtonOptions>(([info, role]) => ({
						type: MessageComponentTypes.BUTTON,
						label: role.name,
						style: info.style,
						customID: `${CUSTOM_ID_PREFIX}${role.id}`,
						emoji: buildEmoji(info.emoji),
					})),
				})),
		};

		try
		{
			if (messageID)
			{
				await channel.messages.edit(messageID, messageOptions);
			}
			else
			{
				await channel.send(messageOptions);
			}
		}
		catch (error: unknown)
		{
			await interaction.reply(
				`Failed to send or edit the message: \`${(error as DiscordAPIError).message}\``,
			);

			throw error;
		}

		const response = messageID
			? `Successfully edited the message for self roles in ${channel}.`
			: `Successfully sent a message for self roles into ${channel}.`
			+ "\nYou may want to delete the old message, if there is one, now.";

		await interaction.reply(response);
	}

	private async cleanup(interaction: CommandInteraction)
	{
		const repo = getConnection().getRepository(SelfRole);

		const selfRoles = await repo.find();
		const missing = selfRoles.filter(r => !interaction.guild!.roles.resolve(r.id));
		if (!missing.length)
		{
			await interaction.reply("There are no roles to prune.", { ephemeral: true });
		}
		else
		{
			await repo.remove(missing);
			await interaction.reply(
				`Removed ${missing.length} no longer existing self assignable roles from the list.`,
			);
		}
	}
}

function isSubCommand(candidate: string): candidate is keyof typeof SubCommand
{
	return Reflect.has(SubCommand, candidate);
}

function buildEmoji(emoji: string | null)
{
	if (!emoji) return undefined;
	if (!emoji.includes("|")) return { name: emoji, id: null };
	const [animated, name, id] = emoji.split("|");

	return {
		animated: animated === "a",
		name,
		id: id as Snowflake,
	};
}

export { SelfRoleModule as Module };
