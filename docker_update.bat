@REM Stop the running container
docker stop wings_container

@REM Remove the container
docker rm wings_container

@REM Remove the old image
docker rmi wings_image

@REM Build a new image
docker build -t wings_image .

@REM Run a new container from the new image
docker run -d --name wings_container -d -p 8080:80 wings_image