import "reflect-metadata";

import { Client } from "./structures/Client";
import { Database } from "./handlers/Database";

new Database().start();

const client: Client = new Client({
	allowedMentions: { parse: ["users"] },
});

client
	.login("NzQ4NDkzNjkyNTExNDUzMjc1.X0ePEg.cwdGJcWMOQrOkBs8yXg0tn06TPI")
	.catch(() => process.exit(1));
