import * as gcp from '@pulumi/gcp'
import * as pulumi from '@pulumi/pulumi'

export const projectName = 'dodo-api'

export const imageName = pulumi.interpolate`asia.gcr.io/${gcp.config.project}/${projectName}:latest`
