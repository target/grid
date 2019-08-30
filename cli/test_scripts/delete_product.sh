#!/bin/bash
docker cp ./delete.yaml gridd:/
docker exec gridd /bin/bash -c 'grid product delete delete.yaml;'
