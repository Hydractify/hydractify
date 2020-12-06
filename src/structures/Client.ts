import { Client as DJSClient, ClientOptions } from "discord.js";
import { join } from "path";
import { readdir } from "fs/promises";

import { IModule } from "./Module";

export class Client extends DJSClient
{
	public constructor(options: ClientOptions)
	{
		super(options);
		this.loadModules();
	}

	private async loadModules(): Promise<void>
	{
		const path: string = join(__dirname, "..", "modules");
		const files: string[] = await readdir(path);

		for (const file of files)
		{
			const moduleConstructor = require(join(path, file)).Module as IModule;

			new moduleConstructor({ client: this }).start();
		}
	}
}
