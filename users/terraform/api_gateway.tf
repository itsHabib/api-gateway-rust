# API
resource "aws_api_gateway_rest_api" "users" {
  name        = "Users API"
  description = "Users API created in content"
}

resource "aws_api_gateway_resource" "users" {
  rest_api_id = "${aws_api_gateway_rest_api.users.id}"
  parent_id   = "${aws_api_gateway_rest_api.users.root_resource_id}"
  path_part = "users"
}

resource "aws_api_gateway_method" "get_users" {
  rest_api_id   = "${aws_api_gateway_rest_api.users.id}"
  resource_id   = "${aws_api_gateway_resource.users.id}"
  http_method   = "GET"
  authorization = "NONE"
}

resource "aws_api_gateway_method" "post_users" {
  rest_api_id   = "${aws_api_gateway_rest_api.users.id}"
  resource_id   = "${aws_api_gateway_resource.users.id}"
  http_method   = "POST"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "get_users" {
  rest_api_id = "${aws_api_gateway_rest_api.users.id}"
  resource_id = "${aws_api_gateway_resource.users.id}"
  http_method = "${aws_api_gateway_method.get_users.http_method}"

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = "${aws_lambda_function.users.invoke_arn}"
}

resource "aws_api_gateway_integration" "post_users" {
  rest_api_id = "${aws_api_gateway_rest_api.users.id}"
  resource_id = "${aws_api_gateway_resource.users.id}"
  http_method = "${aws_api_gateway_method.post_users.http_method}"

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = "${aws_lambda_function.users.invoke_arn}"
}

resource "aws_api_gateway_deployment" "users" {
  depends_on = [
    "aws_api_gateway_integration.get_users",
    "aws_api_gateway_method.get_users",
    "aws_api_gateway_integration.post_users",
    "aws_api_gateway_method.post_users"
  ]

  rest_api_id = "${aws_api_gateway_rest_api.users.id}"
  stage_name = "${var.env}"
}
