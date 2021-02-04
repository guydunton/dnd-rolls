#!/bin/bash -eu

# Fetch all wasm files in bucket
WASM_FILES=$(aws s3api list-objects --bucket dnd-rolls-static-site --query "Contents[?contains(Key, '.wasm')].Key" --output text)

# Set each files metadata "Content-Type" to "application/wasm"
for FILE in $WASM_FILES
do
    aws s3 cp "./dist/$FILE" s3://dnd-rolls-static-site --content-type application/wasm
done
