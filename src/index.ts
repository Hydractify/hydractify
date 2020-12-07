import "reflect-metadata";

import { Client } from "./structures/Client";
import { Database } from "./handlers/Database";

const { token } = require("../config.json");

new Database().start();

const client: Client = new Client({
	allowedMentions: { parse: ["users"] },
	ws: {
		intents: [
			"GUILDS",
			"DIRECT_MESSAGES",
			"GUILD_MESSAGES",
			"GUILD_MESSAGE_REACTIONS",
			"GUILD_MEMBERS",
		],
	},
	partials: ["MESSAGE", "REACTION"],
});

client.on("ready", () => console.log("Ready", client.user!.tag));

client
	.login(token)
	.catch(() => process.exit(1));
