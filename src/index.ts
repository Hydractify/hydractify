import "reflect-metadata";

import { Client } from "./structures/Client";
import { Database } from "./handlers/Database";

const { token } = require("../config.json");

new Database().start();

const client: Client = new Client({
	allowedMentions: { parse: ["users"] },
});

client
	.login(token)
	.catch(() => process.exit(1));
