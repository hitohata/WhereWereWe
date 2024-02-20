#!/usr/bin/env node
import * as cdk from "aws-cdk-lib";
import "source-map-support/register";
import { FunctionStack } from "../lib/function-stack";
import { InterfaceStack } from "../lib/interface-stack";
import { StaticStack } from "../lib/static-stack";
import { stageName } from "../util/utils";

const stage = stageName();

const app = new cdk.App();
const staticAssets = new StaticStack(app, `www-static-assets-${stage}-stack`, {
	stage,
});
const functionAssets = new FunctionStack(
	app,
	`www-function-assets-${stage}-stack`,
	{
		stage,
	},
);
new InterfaceStack(app, `www-interface-assets-${stage}-stack`, {
	stage,
	userPool: staticAssets.userPool,
	lambdaFunction: functionAssets.dummyFunction,
});
