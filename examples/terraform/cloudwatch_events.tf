
resource "aws_iam_role" "events-target-role" {
    name = "${var.shared_short_prefix}-events-target-role"
    assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "",
      "Effect": "Allow",
      "Principal": {
        "Service": "events.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF
}

resource "aws_iam_policy_attachment" "events-target-policy-attachment" {
    name = "${var.shared_short_prefix}-events-target-policy-attachment"
    roles = ["${aws_iam_role.events-target-role.name}"]
    policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonEC2ContainerServiceEventsRole"
}


output "events_target_role_arn" {
    value = "${aws_iam_role.events-target-role.arn}"
}
