terraform {
  backend "s3" {
    bucket = "serverless-rust-state"
    key    = "basic/terraform.tfstate"
    region = "us-east-1"
  }
}

provider "aws" {
  region = "us-east-1"
}

