#!/usr/bin/env bash

set -e

product='authorization-service'


operation() {
    local stack_name="$1"
    aws cloudformation describe-stacks --stack-name "$stack_name" &> /dev/null
    if [ "$?" -eq 0 ]; then
        echo -n update
    else
        echo -n create
    fi
}

provision() {
    local stack_name="${product}-${1}"
    local config_file="cf-${1}.yaml"
    local op="$(operation $stack_name)"
    aws cloudformation "${op}-stack" \
        --stack-name $stack_name \
        --template-body "file://${config_file}" \
        --capabilities CAPABILITY_IAM \
    || echo "$stack_name not provisioned: $?" 1>&2
}

cd "$(dirname "$0")"

export AWS_DEFAULT_REGION=us-east-1

# Infrastructure
provision responder
provision web

