
resource "aws_dynamodb_table" "table" {
  name           = "users"
  billing_mode   = "PROVISIONED"
  read_capacity  = 5
  write_capacity = 5
  hash_key       = "username"

  attribute {
    name = "username"
    type = "S"
  }
}

