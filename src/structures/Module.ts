import { Client, ClientEvents } from "discord.js";

export abstract class Module
{
	private readonly _client: Client;
	private readonly _eventName: keyof ClientEvents;
	private _started: boolean = false;

	public readonly enabled: boolean;

	public constructor({ client, enabled, eventName }: IModuleConfig)
	{
		this._client = client;
		this.enabled = enabled;
		this._eventName = eventName;

		this.handle = this.handle.bind(this);
	}

	protected abstract handle(...args: unknown[]): void | Promise<void>;

	public start(): void
	{
		if (!this.enabled) return;
		if (this._started) return;
		this._started = true;

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
	eventName: keyof ClientEvents;
}
