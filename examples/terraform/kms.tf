
resource "aws_kms_key" "kms-key" {
  policy = "${data.template_file.kms-key-policy.rendered}"

  enable_key_rotation = false
}

data "template_file" "kms-key-policy" {
  template = "${file("kms-key-policy.json.tpl")}"

  vars {
    aws_account_id = "${data.aws_caller_identity.aws-identity.account_id}"
  }
}

resource "aws_kms_alias" "kms-alias" {
  name          = "alias/${var.shared_short_prefix}"
  target_key_id = "${aws_kms_key.kms-key.key_id}"
}


data "aws_iam_policy_document" "kms-iam-policy-document-admin" {
  statement {
    sid = "AllowUseOfTheKey"

    actions = [

      "kms:Encrypt",
      "kms:Decrypt",
      "kms:ReEncrypt*",
      "kms:GenerateDataKey*",
      "kms:DescribeKey",
    ]

    resources = ["${aws_kms_key.kms-key.arn}"]
  }
}

resource "aws_iam_policy" "kms-iam-policy-admin" {
  name        = "${var.shared_short_prefix}-kms-admin"
  path        = "/"
  description = ""
  policy      = "${data.aws_iam_policy_document.kms-iam-policy-document-admin.json}"
}

# resource "aws_iam_policy_attachment" "kms-iam-policy-attachment-admin" {
#   name = "developers-attachment-${var.shared_prefix}-kms"
# 
#   users  = []
#   groups = []
#   roles  = []
# 
#   policy_arn = "${aws_iam_policy.kms-iam-policy-admin.arn}"
# }


data "aws_iam_policy_document" "kms-iam-policy-document-decrypt" {
  statement {
    sid = "AllowUseOfTheKey"

    actions = [

      "kms:Decrypt"
    ]

    resources = ["${aws_kms_key.kms-key.arn}"]
  }
}

resource "aws_iam_policy" "kms-iam-policy-decrypt" {
  name        = "${var.shared_short_prefix}-kms-decrypt"
  path        = "/"
  description = ""
  policy      = "${data.aws_iam_policy_document.kms-iam-policy-document-decrypt.json}"
}


output "kms_key_id" {
  value = "${aws_kms_key.kms-key.key_id}"
}
