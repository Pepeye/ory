#!/bin/bash

set -o errexit
set -o pipefail
set -o nounset
set -x

# hydra
docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli:latest generate -i https://raw.githubusercontent.com/ory/hydra/v1.1.1/docs/api.swagger.json --package-name hydra --library reqwest -g rust -o /local/hydra
# docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli:latest generate -i https://raw.githubusercontent.com/ory/hydra/v1.1.1/spec/api.json --package-name hydra --library reqwest -g rust -o /local/hydra

# kratos
# docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli:latest generate -i https://raw.githubusercontent.com/ory/kratos/29c060bd348733eeafee98d5f255c737a8cbcad0/spec/swagger.json --package-name kratos --library reqwest -g rust -o /local/kratos
docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli:latest generate -i https://raw.githubusercontent.com/ory/kratos/v0.7.6-alpha.1/spec/swagger.json --package-name kratos --library reqwest -g rust -o /local/kratos

# oathkeeper
docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli:latest generate -i https://raw.githubusercontent.com/ory/oathkeeper/v0.38.16-beta.1/spec/api.json --package-name oathkeeper --library reqwest -g rust -o /local/oathkeeper

# keto
docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli:latest generate -i https://raw.githubusercontent.com/ory/keto/v0.7.0-alpha.0/spec/swagger.json --package-name keto --library reqwest -g rust -o /local/keto