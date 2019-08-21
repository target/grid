#!/bin/bash
docker cp /Users/z003bz5/blockchain/grid/cli/test_scripts/delete.yaml gridd:/
docker exec gridd /bin/bash -c 'grid product delete delete.yaml;'
