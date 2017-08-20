provider "aws" {
    region = "${var.aws_region}"
    access_key = "${var.aws_access_key}"
    secret_key = "${var.aws_access_secret}"
}

data "aws_caller_identity" "aws-identity" { }

output "aws_region" {
    value = "${var.aws_region}"
}
