#!/bin/bash

app_uri=$1
item_name="build a todo app"

### Get the page test

get_test () {
	page=$(curl -s -XGET $app_uri)
  get_content=$(<./golden_master.html)

	if ! grep -q "$expected_content" <<< "$page"; then
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
	page=$(curl -L -s -F "item=$item_name" $app_uri)
	expected_content=$item_name

	if ! grep -q "$expected_content" <<< "$page"; then
		global_failures=true
		printf "\tNew todo test... FAILED\n"
		printf "Expected $page\n"
		printf "to contain $expected_content\n"
	else
		printf "\tNew todo test... PASSED\n"
	fi
}

### Complete todo test
complete_todo_test () {
	page=$(curl -L -s -F "item=$item_name" "$app_uri/done")
	expected_content="<s>$item_name</s>"

	if ! grep -q "$expected_content" <<< "$page"; then
		global_failures=true
		printf "\tComplete todo test... FAILED\n"
		printf "Expected $page\n"
		printf "to contain $expected_content\n"
	else
		printf "\tComplete todo test... PASSED\n"
	fi
}

### Delete todo test
delete_todo_test () {
	page=$(curl -L -s -F "item=$item_name" "$app_uri/delete")
	unexpected_content=$item_name

	if grep -q "$unexpected_content" <<< "$page"; then
		global_failures=true
		printf "\tDelete todo test... FAILED\n"
		printf "Expected $page\n"
		printf "not to contain $unexpected_content\n"
	else
		printf "\tDelete todo test... PASSED\n"
	fi
}

### RESULTS

printf "\nRESULTS\n"
get_test
new_todo_test
complete_todo_test
delete_todo_test
