import { Stack, StackProps } from 'aws-cdk-lib'
import { Effect, PolicyStatement } from 'aws-cdk-lib/aws-iam'
import { Code, Function, Runtime } from 'aws-cdk-lib/aws-lambda'
import { Construct } from 'constructs'
import { BuildConfig } from '../getBuildConfig'


export class DangleDestroyerStack extends Stack {
    //@ts-ignore
    constructor ( scope: Construct, id: string, buildConfig: BuildConfig, props?: StackProps ) {
        super( scope, id, props )

        const dangleDestroyerLambda = new Function( this, 'dangleDestroyerLambda', {
            code: Code.fromAsset(
                'lambda/compiled'
            ),
            runtime: Runtime.PROVIDED_AL2,
            handler: 'not.required',
        } )

        dangleDestroyerLambda.addToRolePolicy(
            new PolicyStatement( {
                effect: Effect.ALLOW,
                actions: [
                    'route53:GetHostedZone*',
                    'route53:ListHostedZones*',
                    'route53:ListResourceRecordSets'
                ],
                resources: [ '*' ]
            } )
        )




    }
}
