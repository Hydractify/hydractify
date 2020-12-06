import { Entity, PrimaryColumn, Column } from "typeorm";

@Entity()
export default class Message
{
	@PrimaryColumn()
	id: string;

	@Column()
	authorId: string;

	@Column("text")
	content: string;

	@Column()
	createdAt: Date;
}
