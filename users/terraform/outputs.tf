output "bucket" {
  value = "${aws_s3_bucket.bundles.id}"
}

output "bundle" {
  value = "${aws_s3_bucket_object.bundle.id}"
}

output "handler_arn" {
  value = "${aws_lambda_function.users.arn}"
}

output "base_url" {
  value = "${aws_api_gateway_deployment.users.invoke_url}"
}

