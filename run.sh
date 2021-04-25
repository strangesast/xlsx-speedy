#!/bin/bash

dirname=${1:-python}
if [ -f "$dirname/Dockerfile" ]; then
  echo "running ${dirname}"
  dirname=$(realpath $dirname)
  image=$(docker build -f ${dirname}/Dockerfile --build-arg tag=latest -q $dirname)
  docker run --rm -it -v $dirname/../schedule.xlsm:/tmp/schedule.xlsm -e FILEPATH=/tmp/schedule.xlsm $image
fi
