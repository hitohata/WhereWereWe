import * as path from "node:path";
import { Stack, StackProps } from "aws-cdk-lib";
import { Runtime } from "aws-cdk-lib/aws-lambda";
import { NodejsFunction } from "aws-cdk-lib/aws-lambda-nodejs";
import { Construct } from "constructs";

interface IProps extends StackProps {
	readonly stage: string;
}

export class FunctionStack extends Stack {
	readonly dummyFunction: NodejsFunction;
	constructor(scope: Construct, id: string, props: IProps) {
		super(scope, id, props);
		const { stage } = props;

		this.dummyFunction = new NodejsFunction(this, "dummy-function", {
			functionName: `www-dummy-function-${stage}-function`,
			entry: path.join(__dirname, "../test-lambda/src/index.ts"),
			runtime: Runtime.NODEJS_20_X,
			depsLockFilePath: path.join(
				__dirname,
				"../test-lambda/package-lock.json",
			),
			projectRoot: path.join(__dirname, "../test-lambda"),
			handler: "lambdaHandler",
		});
	}
}
