import { Entity, PrimaryColumn, Column } from "typeorm";

@Entity()
export default class Starboard
{
	@PrimaryColumn()
	messageId: string;

	@Column({
		nullable: true,
	})
	starboardId?: string; // ID of the message in the starboard channel

	@Column()
	stars: number;
}
