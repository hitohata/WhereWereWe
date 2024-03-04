import * as cdk from "aws-cdk-lib";
import {UserPool} from "aws-cdk-lib/aws-cognito";
import {Bucket, BucketEncryption} from "aws-cdk-lib/aws-s3";
import {Construct} from "constructs";
import {Stage} from "../util/utils";

interface IProps extends cdk.StackProps {
	stage: Stage;
}

export class StaticStack extends cdk.Stack {
	private readonly stage: Stage;
	readonly pictureBucket: Bucket;
	readonly userPool: UserPool;

	constructor(scope: Construct, id: string, props: IProps) {
		super(scope, id, props);
		const { stage } = props;

		this.stage = stage;

		this.pictureBucket = this.pictureBucketConstruct();
		this.userPool = this.userPoolDefinition();
	}

	pictureBucketConstruct(): Bucket {
		return new Bucket(this, "WWWBucket", {
			bucketName: `www-bucket-${this.stage}`,
			encryption: BucketEncryption.S3_MANAGED,
			publicReadAccess: false,
			removalPolicy: cdk.RemovalPolicy.DESTROY,
			autoDeleteObjects: true,
		});
	}

	userPoolDefinition() {
		return new UserPool(this, "user-pool", {
			userPoolName: `www-user-pool-${this.stage}`,
		});
	}
}
