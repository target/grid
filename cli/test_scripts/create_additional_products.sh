#!/bin/bash
docker cp /Users/z003bz5/blockchain/grid/cli/test_scripts/create_additional.yaml gridd:/
docker exec gridd /bin/bash -c 'grid product create create_additional.yaml;'
