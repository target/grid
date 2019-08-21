#!/bin/bash
docker cp /Users/z003bz5/blockchain/grid/cli/test_scripts/update.yaml gridd:/
docker exec gridd /bin/bash -c 'grid product update update.yaml;'
