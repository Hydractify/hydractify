module.exports = {
	env: {
		node: true,
	},
	extends: [
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
	plugins: [
		"@typescript-eslint",
	],
	root: true,
	rules: {
		"comma-dangle": ["error", "always-multiline"],
		quotes: ["error", "double", { avoidEscape: true }],
		"brace-style": ["error", "allman"],
		curly: ["error", "multi-line"],
		"quote-props": ["error", "as-needed"],
		semi: ["error", "always"],
		"no-trailing-spaces": "error",
		eqeqeq: ["error", "always"],
		"@typescript-eslint/no-non-null-assertion": "off",
		"@typescript-eslint/no-var-requires": "off",

		// note you must disable the base rule as it can report incorrect errors
		"no-unused-vars": "off",
		"@typescript-eslint/no-unused-vars": ["error", { varsIgnorePattern: "^_", argsIgnorePattern: "^_" }],
	},
};
