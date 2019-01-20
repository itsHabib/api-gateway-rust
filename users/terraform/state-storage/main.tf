provider "aws" {
  region = "us-east-1"
}

resource "aws_s3_bucket" "state-storage" {
  bucket = "serverless-rust-state"

  versioning {
    enabled = true
  }
}

