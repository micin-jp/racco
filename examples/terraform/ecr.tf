
resource "aws_ecr_repository" "repo-nginx" {
  name = "${var.shared_short_prefix}/nginx"
}

resource "aws_ecr_repository" "repo-echo" {
  name = "${var.shared_short_prefix}/echo"
}


output "repository_url_nginx" {
    value = "${aws_ecr_repository.repo-nginx.repository_url}"
}

output "repository_url_echo" {
    value = "${aws_ecr_repository.repo-echo.repository_url}"
}

