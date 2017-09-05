
resource "aws_vpc" "main" {
    cidr_block           = "10.0.0.0/16"
    enable_dns_hostnames = true
    enable_dns_support   = true
    instance_tenancy     = "default"

    tags {
        "Name" = "${var.shared_short_prefix}"
    }
}


##
# public subnet
resource "aws_route_table" "route-table-public" {
    vpc_id     = "${aws_vpc.main.id}"

    route {
        cidr_block = "0.0.0.0/0"
        gateway_id = "${aws_internet_gateway.gateway-main.id}"
    }
    tags {
        "Name" = "${var.shared_short_prefix}-public"
    }
}

resource "aws_internet_gateway" "gateway-main" {
    vpc_id = "${aws_vpc.main.id}"

    tags {
        "Name" = "${var.shared_short_prefix}"
    }
}


resource "aws_eip" "nat-eip" {
    vpc = true
    depends_on = ["aws_internet_gateway.gateway-main"]
}

resource "aws_nat_gateway" "nat-main" {
    allocation_id = "${aws_eip.nat-eip.id}"
    subnet_id = "${aws_subnet.subnet-public-az-a.id}"
}


resource "aws_subnet" "subnet-public-az-a" {
    vpc_id                  = "${aws_vpc.main.id}"
    cidr_block              = "10.0.10.0/24"
    availability_zone       = "${var.aws_az_a}"
    map_public_ip_on_launch = false

    tags {
        "Name" = "${var.shared_short_prefix}-public-az-a"
    }
}

resource "aws_route_table_association" "public-az-a-assoc" {
    route_table_id = "${aws_route_table.route-table-public.id}"
    subnet_id = "${aws_subnet.subnet-public-az-a.id}"
}

resource "aws_subnet" "subnet-public-az-c" {
    vpc_id                  = "${aws_vpc.main.id}"
    cidr_block              = "10.0.15.0/24"
    availability_zone       = "${var.aws_az_c}"
    map_public_ip_on_launch = false

    tags {
        "Name" = "${var.shared_short_prefix}-public-az-c"
    }
}

resource "aws_route_table_association" "public-az-c-assoc" {
    route_table_id = "${aws_route_table.route-table-public.id}"
    subnet_id = "${aws_subnet.subnet-public-az-c.id}"
}


##
# private subnet

resource "aws_route_table" "route-table-private" {
    vpc_id     = "${aws_vpc.main.id}"

    route {
        cidr_block = "0.0.0.0/0"
        nat_gateway_id = "${aws_nat_gateway.nat-main.id}"
    }
    tags {
        "Name" = "${var.shared_short_prefix}-private"
    }
}

resource "aws_subnet" "subnet-private-az-a" {
    vpc_id                  = "${aws_vpc.main.id}"
    cidr_block              = "10.0.20.0/24"
    availability_zone       = "${var.aws_az_a}"
    map_public_ip_on_launch = false

    tags {
        "Name" = "${var.shared_short_prefix}-private-az-a"
    }
}

resource "aws_route_table_association" "private-az-a-assoc" {
    route_table_id = "${aws_route_table.route-table-private.id}"
    subnet_id = "${aws_subnet.subnet-private-az-a.id}"
}

resource "aws_subnet" "subnet-private-az-c" {
    vpc_id                  = "${aws_vpc.main.id}"
    cidr_block              = "10.0.30.0/24"
    availability_zone       = "${var.aws_az_c}"
    map_public_ip_on_launch = false

    tags {
        "Name" = "${var.shared_short_prefix}-private-az-c"
    }
}

resource "aws_route_table_association" "private-az-c-assoc" {
    route_table_id = "${aws_route_table.route-table-private.id}"
    subnet_id = "${aws_subnet.subnet-private-az-c.id}"
}

