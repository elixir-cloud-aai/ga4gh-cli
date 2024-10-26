## Motivation
You will only need to use the script to build models when any of the GA4GH API's [OpenAPI](https://www.openapis.org/) specs are updated. The models are being used to return the result of any API call in the correct format, in the correct structs.

## Prerequisites and Dependencies:
1. OpenAPI Generator CLI: Ensure that OpenAPI Generator CLI 7.9 is installed. You can follow the official installation instructions provided in the OpenAPI Generator documentation:
    [OpenAPI Generator Installation Guide](https://openapi-generator.tech/docs/installation/)

2. Java Development Kit (JDK)
    The OpenAPI generator CLI requires Java to run. Ensure that you have the JDK installed on your system.
    You can install the JDK using the following command:
    - On Ubuntu/Debian:
        ```sh
        sudo apt-get update
        sudo apt-get install default-jdk
        ```
    - On Fedora:
        ```sh
        sudo dnf install java-11-openjdk
        ```
    - On macOS (using Homebrew):
        ```sh
        brew install openjdk@11
        ```

## Usage

### Option 1
1. Clone the repository:
    ```sh
    git clone https://github.com/elixir-cloud-aai/ga4gh-sdk.git
    ```
2. Update the specs of the APIs you want to change in `./utils/build-models.sh`:
    ```sh
    declare -A SPECS
    SPECS["serviceinfo"]="new serviceinfo specs"
    SPECS["tes"]="new tes openapi specs"
    ```
3. Run the following command to automatically generate models using OpenAPI specifications:
    ```sh
    bash ./utils/build_models_wrapper.sh
    ```

### Option 2
To run the script, use the following command:
```sh
./build_models.sh <OPENAPI_SPEC_PATH> <API_NAME> <DESTINATION_DIR>
