#!/bin/bash

### GET test

app_uri=$1
page=$(curl -XGET $app_uri)

fail_get=1

get_content="h1"
if grep -q "h1" <<< "$page"; then
  fail_get=0
fi

### RESULTS

printf "\nRESULTS\n"

if [[ $fail_get = 1 ]]; then
  printf "GET test... FAILED"
	printf "Expected $page"
	printf "to contain $get_content"
else
  printf "\tGET test... PASSED\n"
fi
