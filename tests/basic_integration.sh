#!/bin/bash

trap "killall primitive_web" EXIT

cargo build --release
cargo run --release &

sleep 3

curl -X POST -F 'image=@tests/test.png' -F 'seed=1656301869264' -F 'num_shapes=100' localhost:8000/api/submit > request_result.txt

REQUEST_ID=$(python -c "import json; f = open('request_result.txt'); d = json.load(f); print(d['request_id']);")
rm request_result.txt

COUNT=0

while :
do
    curl localhost:8000/api/get_result/$REQUEST_ID > actual.svg

    if cmp --silent -- "actual.svg" "tests/expected.svg"; then
        rm actual.svg
        exit 0
    fi

    COUNT=$((COUNT+1))
    if [ $COUNT -eq 60 ]; then
        rm actual.svg
        exit 42
    fi
    sleep 1
done
