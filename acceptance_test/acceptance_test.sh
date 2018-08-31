#!/bin/bash

app_uri=$1

### Get the page test

get_test () {
	page=$(curl -s -XGET $app_uri)
	get_content="h1"

	if ! grep -q "h1" <<< "$page"; then
		pass_get=false
		global_failures=true
	fi

	if [[ $pass_get = false ]]; then
		printf "\tGET test... FAILED\n"
		printf "Expected $page\n"
		printf "to contain $get_content\n"
	else
		printf "\tGET test... PASSED\n"
	fi
}

### New todo test
new_todo_test () {
	page=$(curl -L -s -F 'item=build todo app' $app_uri)
	expected_content="build todo app"

	if ! grep -q "$expected_content" <<< "$page"; then
		global_failures=true
		printf "\tNew todo test... FAILED\n"
		printf "Expected $page\n"
		printf "to contain $expected_content\n"
	else
		printf "\tNew todo test... PASSED\n"
	fi
}

### RESULTS

printf "\nRESULTS\n"
get_test
new_todo_test
