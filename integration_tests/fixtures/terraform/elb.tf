
resource "aws_alb" "lb" {
    name            = "${var.shared_short_prefix}"
    idle_timeout    = 60
    internal        = false
    security_groups = ["${aws_security_group.cluster-lb-security-group.id}"]
    subnets         = ["${aws_subnet.subnet-public-az-a.id}", "${aws_subnet.subnet-public-az-c.id}"]

    enable_deletion_protection = false
}

resource "aws_alb_listener" "lb-listener" {
   load_balancer_arn = "${aws_alb.lb.arn}"

   port = "80"
   protocol = "HTTP"

   default_action {
       target_group_arn = "${aws_alb_target_group.lb-target-group.arn}"
       type = "forward"
   }
}

resource "aws_alb_target_group" "lb-target-group" {
    name     = "${var.shared_short_prefix}"
    port     = 80
    protocol = "HTTP"
    vpc_id   = "${aws_vpc.main.id}"
    health_check {
        path = "/"
        interval = 10
        healthy_threshold = 2
    }
}

resource "aws_security_group" "cluster-lb-security-group" {
    name   = "${var.shared_short_prefix}-cluster-lb"
    vpc_id = "${aws_vpc.main.id}"

    ingress {
        from_port       = 80
        to_port         = 80
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

output "alb_target_group_arn" {
    value = "${aws_alb_target_group.lb-target-group.arn}"
}

