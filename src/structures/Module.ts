import { Client } from "discord.js";

export abstract class Module
{
	private readonly _client: Client;
	private readonly _eventName: string;

	public readonly enabled: boolean;

	public constructor({ client, enabled, eventName }: IModuleConfig)
	{
		this._client = client;
		this.enabled = enabled;
		this._eventName = eventName;

		this.handle = this.handle.bind(this);
	}

	protected abstract handle(...args: any[]): void | Promise<void>;

	public start(): void
	{
		if (!this.enabled) return;

		this._client.on(this._eventName, this.handle);
	}
}

export interface IModule
{
	new(options: { client: Client }): Module;
}

export interface IModuleConfig
{
	client: Client;
	enabled: boolean;
	eventName: string;
}
