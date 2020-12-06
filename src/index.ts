import "reflect-metadata";

import { Client } from "./structures/Client";
import { Database } from "./handlers/Database";

new Database().start();

const client: Client = new Client({
	allowedMentions: { parse: ["users"] },
});

client
	.login("SUPER_SECRET_NEW_TOKEN")
	.catch(() => process.exit(1));
