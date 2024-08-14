#!/bin/bash

# Ensure the correct number of arguments
if [ "$#" -ne 3 ]; then
    echo "Usage: $0 <OPENAPI_SPEC_PATH> <API_NAME> <DESTINATION_DIR>"
    exit 1
fi

# Get the parameters
OPENAPI_SPEC_PATH="$1"
API_NAME="$2"
DESTINATION_DIR="$3"

# Define constants
SED_RULE="s/^use\s\+crate::models\s*;/#![allow(unused_imports)]\n#![allow(clippy::empty_docs)]\nuse crate::$API_NAME::models;/"
TEMP_OUTPUT_DIR=$(mktemp -d)  # Define the temporary output directory for the OpenAPI generator

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

generate_openapi_models() {
    # Remove the temporary directory at the end of the script
    trap 'rm -rf "$TEMP_OUTPUT_DIR"' EXIT
    
    mkdir -p ~/bin/openapitools
    curl https://raw.githubusercontent.com/OpenAPITools/openapi-generator/master/bin/utils/openapi-generator-cli.sh > ~/bin/openapitools/openapi-generator-cli
    chmod u+x ~/bin/openapitools/openapi-generator
    export PATH=$PATH:~/bin/openapitools/

    openapi-generator-cli version

    # Run the OpenAPI generator CLI using the JAR file
    openapi-generator-cli generate -g rust \
        -i "$OPENAPI_SPEC_PATH" \
        -o "$TEMP_OUTPUT_DIR" \
        --additional-properties=useSingleRequestParameter=true 

    # Check if the generation was successful
    if [ $? -ne 0 ]; then
        echo "OpenAPI generation failed. Check the verbose output for details."
        exit 1
    fi

    # Remove the openapitools.json file
    rm -f "$TEMP_OUTPUT_DIR/openapitools.json"
    
    echo "TEMP_OUTPUT_DIR is $TEMP_OUTPUT_DIR"

    # Modify the import statements in each generated file
    find "$TEMP_OUTPUT_DIR" -name '*.rs' > /dev/null
    if [ $? -ne 0 ]; then
        echo "Error: 'find' command failed."
        exit 1
    fi

    for file in $(find "$TEMP_OUTPUT_DIR" -name '*.rs'); do
        if [[ "$OSTYPE" == "darwin"* ]]; then
            # macOS (BSD) sed syntax
            sed -i '' "$SED_RULE" "$file"
        else
            # Linux (GNU) sed syntax
            sed -i "$SED_RULE" "$file"
        fi
    done

    rm -rf "$DESTINATION_DIR/models"
    mkdir -p "$DESTINATION_DIR"
    cp -r "$TEMP_OUTPUT_DIR/src/models" "$DESTINATION_DIR"

    echo "OpenAPI generation complete. Models copied to $DESTINATION_DIR"
}

# Call the function to generate models
generate_openapi_models "$OPENAPI_SPEC_PATH" "$API_NAME" "$DESTINATION_DIR"
