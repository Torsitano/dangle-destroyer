#!/usr/bin/env node
import 'source-map-support/register'
import * as cdk from 'aws-cdk-lib'
import { DangleDestroyerStack } from './stacks/DangleDestroyerStack'
import { getBuildConfig } from './getBuildConfig'

const app = new cdk.App()


const buildConfig = getBuildConfig( app )

new DangleDestroyerStack( app, 'DangleDestroyerStack', buildConfig, {

} )