#!/bin/bash

root=$(realpath $(dirname $0))
dirname=${1:-python}
if [ -f "$root/$dirname/Dockerfile" ]; then
  if [ -t 1 ] ; then
    echo "running $dirname"
  fi
  base=$root/$dirname
  image=$(docker build -f ${base}/Dockerfile --build-arg tag=latest -q $base)
  docker run --rm -it -v $base/../schedule.xlsm:/tmp/schedule.xlsm -e FILEPATH=/tmp/schedule.xlsm $image
fi
