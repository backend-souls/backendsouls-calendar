#!/bin/bash

CONTAINER_NAME="calendar"

if [ "$(docker ps -a | grep "$CONTAINER_NAME")" ]; then
  docker stop "$CONTAINER_NAME"
  docker rm "$CONTAINER_NAME"
  echo "Removed container: $CONTAINER_NAME."
else
  echo "Container $CONTAINER_NAME does not exist."
fi
