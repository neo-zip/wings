#!/bin/bash

# Stop the running container
docker stop wings_container

# Remove the container
docker rm wings_container

# Remove the old image
docker rmi wings_image

# Build a new image
docker build -t wings_image .

# Run a new container from the new image
docker run -d --name wings_container -d -p 80:80 wings_image