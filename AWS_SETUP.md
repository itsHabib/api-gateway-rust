# AWS Setup

## Using Terraform
We will use terraform to create all the resources needed in AWS. If you are not
interestd in having terraform provision the resources, use the links below to
do it manually. The three main components that will be created will the be the
API gateway resources, lambda function, and S3 buckets. To get started checkout
the apporpriate branch. The `advanced` branch will contain the terraform code
to create what is needed for the YouTube video **(work in progress)**. The
`basic` and `master` branches have the terraform code needed for the blog post.

### Setting up Remote State
To setup remote state navigate to `users/terraform/state-storage` and open
`main.tf`. Rename the bucket name to something of your choosing, S3 bucket names
need to be globally unique. Once that is done from `users/` run `./deploy.sh --state`.
Once the bucket has been created, edit the `bucket` value for the terraform backend in
`users/terraform/main.tf`. Use the same value as the bucket for the state
storage. These are the only changes that should be needed to run the other commands
in the blog post!

## Using the AWS Console
Since the above describes how to use terraform I have not gone through the
steps on provisioning the resources in the AWS console. Instead I've provided
links to documentation written by AWS.
 - [Create a S3 bucket](https://docs.aws.amazon.com/AmazonS3/latest/user-guide/create-bucket.html)
 - [Create a lambda function](https://docs.aws.amazon.com/lambda/latest/dg/getting-started-create-function.html)
 - [API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/welcome.html)
 - [Controlling Access to an API in API Gateway](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-control-access-to-api.html)
 - [DynamoDB Developer Guide](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Introduction.html)

## Cleaning Up
To clean up with terraform, from `users/` run `./destroy.sh` and once that is complete
run `./destroy.sh --state`. Those commands will destroy EVERYTHING that was created with `deploy.sh`.
