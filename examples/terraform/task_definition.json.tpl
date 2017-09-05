[
    {
        "name": "nginx",
        "image": "${repository_url}:${repository_tag}",
        "cpu": 50,
        "memory": 128,
        "memoryReservation": 64,
        "links": [
        ],
        "portMappings": [
            {
                "containerPort": 80
            }
        ],
        "logConfiguration": {
           "logDriver": "awslogs",
           "options": {
               "awslogs-group": "${awslogs_group}",
               "awslogs-region": "${aws_region}",
               "awslogs-stream-prefix": "${awslogs_prefix}"
           }
        },
        "essential": true
    }
]

