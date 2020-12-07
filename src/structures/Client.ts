import { Client as DJSClient, ClientOptions } from "discord.js";
import { join } from "path";
import { readdir } from "fs/promises";

import { IModule, Module } from "./Module";

export class Client extends DJSClient
{
	public readonly modules = new Map<string, Module>();

	public constructor(options: ClientOptions)
	{
		super(options);
		this.loadModules();
		this.startModules();
	}

	private async loadModules(): Promise<void>
	{
		const path: string = join(__dirname, "..", "modules");
		const files: string[] = await readdir(path);

		for (const file of files)
		{
			const moduleConstructor = require(join(path, file)).Module as IModule;
			const module = new moduleConstructor({ client: this });

			this.modules.set(moduleConstructor.name, module);
		}
	}

	private startModules(): void
	{
		for (const module of this.modules.values()) module.start();
	}
}
