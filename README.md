# How to run
`git clone https://github.com/BradMarr/wings.git`

`cd wings`

`chmod +x ./docker_update.sh`

`docker build -t wings_image .`

`docker run -d --name wings_container -p 80:80 wings_image`

to update: `./docker_update.sh`

go to `127.0.0.1`
