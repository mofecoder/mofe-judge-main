#!/bin/bash

docker rm $(docker ps -a | grep 'cafecoder' | awk '{print $1}')