variable "aws_access_key" {}
variable "aws_access_secret" {}

variable "aws_region" {
    default = "ap-northeast-1"
}

variable "aws_az_a" {
    default = "ap-northeast-1a"
}

variable "aws_az_c" {
    default = "ap-northeast-1c"
}

variable "shared_short_prefix" {
    default = "racco-test"
}

variable "instance_num" {
    default = 2
}

variable "container_count" {
    default = 4
}

variable "ami_id" {
    default = "ami-f63f6f91"
}

