
resource "aws_security_group" "cluster-instance-security-group" {
    name        = "${var.shared_short_prefix}-cluster-instance"
    vpc_id      = "${aws_vpc.main.id}"

    ingress {
        from_port       = 0
        to_port         = 65535
        protocol        = "tcp"
        cidr_blocks     = ["0.0.0.0/0"]
    }

    egress {
        from_port       = 0
        to_port         = 0
        protocol        = "-1"
        cidr_blocks     = ["0.0.0.0/0"]
    }
}

resource "aws_instance" "cluster-instance" {
    count = "${var.instance_num}"
    ami                         = "${var.ami_id}"
    availability_zone           = "${var.aws_az_a}"
    ebs_optimized               = false
    instance_type               = "t2.micro"
    monitoring                  = false
    subnet_id                   = "${aws_subnet.subnet-private-az-a.id}"
    vpc_security_group_ids      = ["${aws_security_group.cluster-instance-security-group.id}"]
    associate_public_ip_address = false
    source_dest_check           = true
    iam_instance_profile        = "${aws_iam_instance_profile.cluster-instance-profile.name}"

    root_block_device {
        volume_type           = "gp2"
        volume_size           = 8
        delete_on_termination = true
    }

    tags {
        "Name" = "${var.shared_short_prefix}-host"
    }

    user_data = "${file("user_data.sh")}"
}

