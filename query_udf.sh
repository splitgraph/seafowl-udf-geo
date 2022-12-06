#! /usr/bin/env bash

# Parameters to set
function_name="distance";
function_arguments="52.518611, 13.408056, 55.751667, 37.617778";
host="localhost:8080";

curl -i -H "Content-Type: application/json" $host/q -d@- <<EndOfMessage
{"query": "SELECT $function_name($function_arguments) AS RESULT;"}
EndOfMessage

