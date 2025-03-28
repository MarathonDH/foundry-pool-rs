#!/bin/bash
# Download the json file from https://api.foundryusapool.com/
# Add the `info.version` field
# Run command below.
openapi-generator generate -i ./openapi-spec-6.0.json -g rust -o . --additional-properties=packageName=foundry-pool-rs,preferUnsignedInt=true,bestFitInt=true,useSingleRequestParameter=true