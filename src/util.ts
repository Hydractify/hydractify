/**
 * Split an array into smaller cunks.
 * The original array will _not_ be modified.
 */
export const chunkArray: <T>(input: readonly T[], chunkSize: number) => T[][]
	= <T>(input: readonly T[], chunkSize: number): T[][] =>
	{
		const chunks: T[][] = [];
		const length: number = Math.ceil(input.length / chunkSize);

		for (let i = 0; i < length; undefined)
		{
			chunks.push(input.slice(i * chunkSize, ++i * chunkSize));
		}

		return chunks;
	};
