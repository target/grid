#!/bin/bash
docker cp /Users/z003bz5/blockchain/grid/cli/test_scripts/create.yaml gridd:/
docker exec gridd /bin/bash -c 'grid keygen --force; \
 grid organization create "314156" "target" "target hq" --metadata "gs1_company_prefixes=314"; \
 PUB_KEY="$(cat ~/.grid/keys/root.pub)"; \
 grid agent update "314156" "$PUB_KEY" true --roles "admin" "can_create_product" "can_update_product" "can_delete_product"; \
 grid product create create.yaml;'
