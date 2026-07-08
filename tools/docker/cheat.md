# Docker Cheat Sheet

## Docker Basic

**Check Docker version.**
`docker --version or docker -v`

**Display system-wide information about Docker.**
`docker info`

**Download an image from Docker Hub.**
`docker pull <image_name>`

**List local Docker images.**
`docker images or docker images ls`

**List running containers.**
`docker ps or docker container ls`

**List all containers (including stopped ones**
`docker ps -a or docker container ls -a`

**Create and start a new container from an image.**
`docker run <option> <image_name>`

## Container Lifecycle

**Start a stopped container.**
`docker start <container_name/id>`

**Stop a running container gracefully.**
`docker stop <container_name/id>`
