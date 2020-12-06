import { Entity, PrimaryColumn, Column } from "typeorm";

@Entity()
export default class User
{
	@PrimaryColumn()
	id: string;

	@Column({
		nullable: true,
	})
	storyEula?: Date;
}
