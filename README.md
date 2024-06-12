# How to run
`git clone https://github.com/BradMarr/wings.git`

`docker build -t wings_image .`

`docker run -d --name wings_container -p 80:80 wings_image`

go to `127.0.0.1`
