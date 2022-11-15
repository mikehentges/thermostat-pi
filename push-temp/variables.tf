# Input variable definitions, adjust for your needs

variable "aws_region" {
  description = "AWS region for all resources."

  type    = string
  default = "us-east-2"
}

variable "push_temp_bin_path" {
  description = "The binary path for the push_temp lambda."

  type    = string
  default = "./bootstrap"
}



