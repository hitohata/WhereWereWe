export type Stage = "prod" | "dev";

/**
 * return stage name.
 * The stage information reads from the environment variable.
 */
export const stageName = (): Stage => {
	const stage = process.env.STAGE;

	if (!stage) return "dev";
	if (stage === "prod") return "prod";
	return "dev";
};
