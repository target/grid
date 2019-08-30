#!/bin/bash
docker cp ./update.yaml gridd:/
docker exec gridd /bin/bash -c 'grid product update update.yaml;'
