#!/bin/bash

set -o errexit
set -o pipefail
set -o nounset
set -x

docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli:latest generate -i https://raw.githubusercontent.com/ory/hydra/v1.1.1/docs/api.swagger.json --package-name hydra --library reqwest -g rust -o /local/hydra
