import { AppSyncResolverEvent } from "aws-lambda";

// biome-ignore lint/suspicious/noExplicitAny:
export const lambdaHandler = async (event: AppSyncResolverEvent<any>) => {
	console.log(event);
};
