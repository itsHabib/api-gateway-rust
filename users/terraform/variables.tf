variable "name" {
  description = "application name"
  default     = "serverless-rust"
}

variable "env" {
  description = "deployment environment"
  default     = "dev"
}

variable "tag" {
  description = "app version / commit hash"
}
