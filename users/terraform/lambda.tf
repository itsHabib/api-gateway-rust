# Lambda related resources

# Code bundles
resource "aws_s3_bucket" "bundles" {
  bucket = "${var.name}-${var.env}-bundles"
  versioning {
    enabled = true
  }
}

# bundle is sourced from docker build
resource "aws_s3_bucket_object" "bundle" {
  bucket = "${aws_s3_bucket.bundles.id}"
  key    = "${var.name}-${var.env}-${var.tag}-bundle.zip"
  source = "/bundles/bundle.zip"
}

# lambda permissions
resource "aws_iam_role" "lambda_role" {
  name               = "${var.name}-${var.env}-lambda-role"
  assume_role_policy = "${file("${path.module}/policies/lambda-assume-role.json")}"
}

resource "aws_iam_role_policy" "lambda_role_policy" {
  name   = "${var.name}-${var.env}-lambda-role-policy"
  policy = "${file("${path.module}/policies/lambda-role.json")}"
  role   = "${aws_iam_role.lambda_role.id}"
}

resource "aws_lambda_permission" "api_gateway" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = "${aws_lambda_function.users.arn}"
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.users.execution_arn}/${var.env}/*/*"
}

resource "aws_lambda_function" "users" {
  function_name = "${var.name}-${var.env}-lambda"
  runtime       = "provided"
  handler       = "main.handler"

  role = "${aws_iam_role.lambda_role.arn}"

  s3_bucket         = "${aws_s3_bucket.bundles.id}"
  s3_key            = "${aws_s3_bucket_object.bundle.key}"
  s3_object_version = "${aws_s3_bucket_object.bundle.version_id}"

  depends_on = ["aws_s3_bucket_object.bundle"]
}
