import * as cdk from "aws-cdk-lib";

import * as path from "node:path";
import { Stack } from "aws-cdk-lib";
import * as appsync from "aws-cdk-lib/aws-appsync";
import { UserPool } from "aws-cdk-lib/aws-cognito";
import * as iam from "aws-cdk-lib/aws-iam";
import { NodejsFunction } from "aws-cdk-lib/aws-lambda-nodejs";
import { Stage } from "../util/utils";

interface IProps extends cdk.StageProps {
	readonly stage: Stage;
	readonly userPool: UserPool;
	readonly lambdaFunction: NodejsFunction;
}

export class InterfaceStack extends cdk.Stack {
	constructor(scope: cdk.App, id: string, props: IProps) {
		super(scope, id, props);

		const { stage, userPool, lambdaFunction } = props;

		const api = appSyncEndpoint(this, stage, userPool);

		const role = new iam.Role(this, "lambda-role", {
			assumedBy: new iam.ServicePrincipal("lambda.amazonaws.com"),
		});

		api.grant(role, appsync.IamResource.custom("types/*"), "appsync:GraphQL");
	}
}

const appSyncEndpoint = (stack: Stack, stage: Stage, userPool: UserPool) => {
	return new appsync.GraphqlApi(stack, "gql-api", {
		name: `where-were-we-endpoint-${stage}-app-sync`,
		authorizationConfig: {
			defaultAuthorization: {
				authorizationType: appsync.AuthorizationType.USER_POOL,
				userPoolConfig: {
					userPool: userPool,
				},
			},
			additionalAuthorizationModes: [
				{
					authorizationType: appsync.AuthorizationType.IAM,
				},
			],
		},
		definition: appsync.Definition.fromFile(
			path.join(__dirname, "../graphql/schema.graphql"),
		),
	});
};
