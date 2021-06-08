import { Column, Entity, PrimaryColumn } from "typeorm";


@Entity()
export default class SelfRole
{
	@PrimaryColumn()
	id: string;

	@Column({ type: "int", nullable: false })
	style: number;

	// The format is `a|${name}|${id}` for custom emojis or `${emoji}` for default emojis.
	// THIS IS NOT VALIDATED (because you can only do that by hitting up Discord with some request)
	@Column({ type: "text", nullable: true })
	emoji: string | null;
}
