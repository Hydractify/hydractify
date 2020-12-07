module.exports = {
	env: {
		node: true,
	},
	extends:
	[
		"eslint:recommended",
		"plugin:@typescript-eslint/eslint-recommended",
		"plugin:@typescript-eslint/recommended",
	],
	parser: "@typescript-eslint/parser",
	parserOptions:
	{
		ecmaVersion: 2020,
		sourceType: "module",
	},
	plugins:
	[
		"@typescript-eslint",
	],
	root: true,
	rules:
	{
		/*
		 * ESLint rules
		 */
		"brace-style":
		[
			"error",
			"allman",
		],
		"comma-dangle":
		[
			"warn",
			"always-multiline",
		],
		curly:
		[
			"error",
			"multi-line",
		],
		quotes:
		[
			"error",
			"double",
			{
				avoidEscape: true,
			},
		],
		"quote-props":
		[
			"error",
			"as-needed",
		],
		semi:
		[
			"error",
			"always",
		],
		"no-trailing-spaces": "error",
		indent: "off",
		"no-unexpected-multiline": "off",
		radix: "off",
		"require-atomic-updates": "off",
		/*
		 * TSLint rules
		 */
		"@typescript-eslint/indent":
		[
			"error",
			"tab",
		],
		"@typescript-eslint/no-inferrable-types": "off",
		"@typescript-eslint/no-non-null-assertion": "off",
		"@typescript-eslint/no-var-requires": "off",
	},
};
