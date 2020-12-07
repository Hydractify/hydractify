import { GuildMember } from "discord.js";
import { IModuleConfig, Module } from "../structures/Module";

const { guild_access: { enabled, role } } = require("../../config.json");

class GuildAccessJoin extends Module
{
	public constructor(config: IModuleConfig)
	{
		super({
			client: config.client,
			enabled,
			eventName: "guildMemberAdd",
		});
	}

	public async handle(member: GuildMember): Promise<void>
	{
		if (member.user.bot) return;

		await member.roles.add(role);
	}
}

export { GuildAccessJoin as Module };
