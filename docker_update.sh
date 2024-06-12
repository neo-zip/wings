#!/bin/bash

# Stop the running container
docker stop wings

# Remove the container
docker rm wings

# Remove the old image
docker rmi wings

# Build a new image
docker build -t wings .

# Run a new container from the new image
docker run -d --name wings -d -p 80:80 wings