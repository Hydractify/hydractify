import "reflect-metadata";

import { Client } from "./structures/Client";
import { Database } from "./handlers/Database";

const { token } = require("../config.json");

new Database().start();

const client: Client = new Client({
	allowedMentions: { parse: ["users"] },
});

client.on("ready", () => console.log("Ready", client.user!.tag));

client
	.login(token)
	.catch(() => process.exit(1));
