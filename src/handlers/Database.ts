import { Connection, createConnection, ConnectionOptions } from "typeorm";

export class Database
{
	private readonly options: ConnectionOptions;

	public constructor()
	{
		this.options = {
			type: "postgres",
			host: "localhost",
			username: "hydractify",
			password: "hyfy",
			database: "hydra_bot",
			entities: [
				`${__dirname.replace(/handlers$/, "entity")}/**/*.js`,
			],
			logging: "all",
			synchronize: true,
		};
	}

	public start(): Promise<Connection>
	{
		return createConnection(this.options);
	}
}
