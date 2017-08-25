{
  "Version": "2012-10-17",
  "Id": "key-policy-mogura",
  "Statement": [
    {
      "Sid": "Enable IAM User Permissions",
      "Effect": "Allow",
      "Principal": {"AWS": "arn:aws:iam::${aws_account_id}:root"},
      "Action": "kms:*",
      "Resource": "*"
    }
  ]
}

