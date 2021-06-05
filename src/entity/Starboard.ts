import { Snowflake } from "discord.js";
import { Entity, PrimaryColumn, Column } from "typeorm";

@Entity()
export default class Starboard
{
	@PrimaryColumn()
	messageId: Snowflake;

	@Column({
		nullable: true,
	})
	starboardId?: Snowflake; // ID of the message in the starboard channel

	@Column()
	stars: number;
}
