terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
    archive = {
      source  = "hashicorp/archive"
      version = "~> 2.2.0"
    }
  }

  required_version = "~> 1.0"

  cloud {
    organization = "Hentges-AI"

    workspaces {
      name = "rust-pi-thermostat"
    }
  }
}

provider "aws" {
  region = var.aws_region
}
