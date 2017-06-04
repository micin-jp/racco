
resource "aws_ecs_cluster" "cluster" {
  name = "${var.shared_short_prefix}-cluster"
}

resource "aws_iam_instance_profile" "cluster-instance-profile" {
    name = "${var.shared_short_prefix}-instance-profile"
    roles = ["${aws_iam_role.cluster-instance-role.name}"]
}

resource "aws_iam_role" "cluster-instance-role" {
    name = "${var.shared_short_prefix}-instance-role"
    assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "",
      "Effect": "Allow",
      "Principal": {
        "Service": "ec2.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF
}

resource "aws_iam_policy_attachment" "cluster-instance-policy-attachment" {
    name = "${var.shared_short_prefix}-instance-policy-attachment"
    roles = ["${aws_iam_role.cluster-instance-role.name}"]
    policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonEC2ContainerServiceforEC2Role"
}

resource "aws_iam_role" "cluster-service-role" {
    name = "${var.shared_short_prefix}-service-role"
    assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "",
      "Effect": "Allow",
      "Principal": {
        "Service": "ecs.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF
}

resource "aws_iam_policy_attachment" "cluster-service-policy-attachment" {
    name = "${var.shared_short_prefix}-service-policy-attachment"
    roles = ["${aws_iam_role.cluster-service-role.name}"]
    policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonEC2ContainerServiceRole"
}

resource "aws_cloudwatch_log_group" "log-group" {
  name = "${var.shared_short_prefix}"
  retention_in_days = 14

  tags {
  }
}

resource "aws_iam_role" "task-iam-role" {
  name = "${var.shared_short_prefix}-task-role"
  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "",
      "Effect": "Allow",
      "Principal": {
        "Service": "ecs-tasks.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF
}

output "task_role_arn" {
    value = "${aws_iam_role.task-iam-role.arn}"
}

output "cluster_service_role_arn" {
    value = "${aws_iam_role.cluster-service-role.arn}"
}

output "awslogs_group" {
    value = "${aws_cloudwatch_log_group.log-group.name}"
}
