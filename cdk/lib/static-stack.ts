import * as cdk from "aws-cdk-lib";
import { UserPool } from "aws-cdk-lib/aws-cognito";
import { Bucket } from "aws-cdk-lib/aws-s3";
import { Construct } from "constructs";
import { Stage } from "../util/utils";

interface IProps extends cdk.StackProps {
	stage: Stage;
}

export class StaticStack extends cdk.Stack {
	pictureBucket: Bucket;
	readonly userPool: UserPool;

	constructor(scope: Construct, id: string, props: IProps) {
		super(scope, id, props);
		const { stage } = props;

		this.pictureBucket = pictureBucket(this, stage);
		this.userPool = userPool(this, stage);
	}
}

/**
 * S3 Bucket
 * @param stack
 * @param stage
 */
const pictureBucket = (stack: cdk.Stack, stage: Stage): Bucket => {
	return new Bucket(stack, "picture-bucket", {
		bucketName: `www-picture-${stage}-bucket`,
	});
};

/**
 * user pool
 * @param stack
 * @param stage
 */
const userPool = (stack: cdk.Stack, stage: Stage): UserPool => {
	return new UserPool(stack, "user-pool", {
		userPoolName: `user-pool-${stage}`,
	});
};
