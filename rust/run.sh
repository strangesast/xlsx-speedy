#!/bin/bash
docker run --rm -it -v $(pwd)/../schedule.xlsm:/tmp/schedule.xlsm -e FILEPATH=/tmp/schedule.xlsm $(docker build --build-arg tag=latest -q .)
