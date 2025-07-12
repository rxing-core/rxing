#!/bin/bash

# --- Test Suite Configuration ---

# Path to the compiled command-line tool.
# Adjust this path if your binary is in a different location.
CLI_PATH="../../target/debug/rxing-cli"

# Directory to store generated test files (barcodes, data files).
TEST_DIR="test_artifacts"

# --- Color Definitions for Output ---
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# --- Test Counters ---
tests_run=0
tests_passed=0

# --- Helper Functions ---

# Function to set up the testing environment.
setup() {
    echo -e "${YELLOW}Setting up test environment...${NC}"
    # Ensure the CLI program is compiled and available.
    if [ ! -f "$CLI_PATH" ]; then
        echo -e "${RED}Error: CLI program not found at '$CLI_PATH'.${NC}"
        echo -e "${YELLOW}Please compile the Rust project first (e.g., run 'cargo build').${NC}"
        exit 1
    fi

    # Create a clean directory for test artifacts.
    rm -rf "$TEST_DIR"
    mkdir -p "$TEST_DIR"

    # Create a sample data file for testing.
    echo "Testing data from file." > "$TEST_DIR/sample.txt"
    echo -e "${GREEN}Setup complete.${NC}\n"
}

# Function to run a single test case.
# Arguments:
#   $1: Test ID (e.g., "EN-01")
#   $2: Test Description
#   $3: Command to execute (as a string)
#   $4: Expected exit code (0 for success, non-zero for expected failure)
#   $5: Expected output string to find (use "" for no check)
#   $6: File to check for existence after command runs (use "" for no check)
run_test() {
    ((tests_run++))
    local test_id="$1"
    local description="$2"
    local command="$3"
    local expected_code="$4"
    local expected_output="$5"
    local file_to_check="$6"

    echo -e "▶️  Running Test ${YELLOW}${test_id}:${NC} ${description}"

    # Execute the command and capture its output and exit code.
    output=$(eval "$command" 2>&1)
    exit_code=$?

    # Check 1: Exit Code
    if [ "$exit_code" -ne "$expected_code" ]; then
        echo -e "  ${RED}FAILURE:${NC} Expected exit code ${YELLOW}${expected_code}${NC}, but got ${YELLOW}${exit_code}${NC}."
        echo -e "  ${RED}Output:${NC}\n$output"
        return
    fi

    # Check 2: Expected Output String
    if [ -n "$expected_output" ] && ! echo "$output" | grep -qF "$expected_output"; then
        echo -e "  ${RED}FAILURE:${NC} Expected output to contain '${YELLOW}${expected_output}${NC}'."
        echo -e "  ${RED}Actual Output:${NC}\n$output"
        return
    fi

    # Check 3: File Existence
    if [ -n "$file_to_check" ] && [ ! -f "$file_to_check" ]; then
        echo -e "  ${RED}FAILURE:${NC} Expected file '${YELLOW}${file_to_check}${NC}' to be created, but it was not."
        return
    fi

    # If all checks pass:
    echo -e "  ${GREEN}SUCCESS${NC}"
    ((tests_passed++))
}

# Function to tear down the testing environment.
teardown() {
    echo -e "\n${YELLOW}Tearing down test environment...${NC}"
    rm -rf "$TEST_DIR"
    echo -e "${GREEN}Cleanup complete.${NC}"
}

# --- Test Execution ---

setup

# --- ENCODE Subcommand Tests ---
echo -e "\n--- Testing ENCODE Subcommand ---\n"

# EN-01: Basic QR Code Encoding
run_test "EN-01" "Basic QR Code Generation" \
    "$CLI_PATH $TEST_DIR/qrcode.png encode QR_CODE --data \"Hello, World!\"" \
    0 "Encode successful, saving..." "$TEST_DIR/qrcode.png"

# EN-02: Basic Code 128 Encoding
run_test "EN-02" "Basic Code 128 Generation" \
    "$CLI_PATH $TEST_DIR/code128.png encode CODE_128 --data \"1234567890\"" \
    0 "Encode successful, saving..." "$TEST_DIR/code128.png"

# EN-03: Data Matrix with Dimensions
run_test "EN-03" "Data Matrix with specific dimensions" \
    "$CLI_PATH $TEST_DIR/datamatrix.png encode DATA_MATRIX --data \"Test Data\" --width 250 --height 250" \
    0 "Encode successful, saving..." "$TEST_DIR/datamatrix.png"

# EN-04: Encoding from a data file
run_test "EN-04" "Encoding data from a file" \
    "$CLI_PATH $TEST_DIR/qrcode_from_file.png encode QR_CODE --data-file $TEST_DIR/sample.txt" \
    0 "Encode successful, saving..." "$TEST_DIR/qrcode_from_file.png"

# EN-05: QR Code with High Error Correction
run_test "EN-05" "QR Code with high error correction" \
    "$CLI_PATH $TEST_DIR/qrcode_error_h.png encode QR_CODE --data \"High Error\" --error-correction H" \
    0 "Encode successful, saving..." "$TEST_DIR/qrcode_error_h.png"

# EN-06: Invalid - Missing Data Source
run_test "EN-06" "Invalid command - missing data source" \
    "$CLI_PATH $TEST_DIR/missing.png encode QR_CODE" \
    2 "error: the following required arguments were not provided:" ""

# EN-07: Invalid - Bad Barcode Format
run_test "EN-07" "Invalid command - bad barcode format" \
    "$CLI_PATH $TEST_DIR/invalid.png encode FAKE_FORMAT --data \"test\"" \
    1 " No encoder available for format" ""


# --- DECODE Subcommand Tests ---
echo -e "\n--- Testing DECODE Subcommand ---\n"

# DE-01: Basic QR Code Decoding
run_test "DE-01" "Basic QR Code Decoding" \
    "$CLI_PATH $TEST_DIR/qrcode.png decode" \
    0 "(qrcode) Hello, World!" ""

# DE-02: Basic Code 128 Decoding
run_test "DE-02" "Basic Code 128 Decoding" \
    "$CLI_PATH $TEST_DIR/code128.png decode" \
    0 "(code 128) 1234567890" ""

# DE-03: Decoding data from file-generated QR code
run_test "DE-03" "Decoding a file-based QR Code" \
    "$CLI_PATH $TEST_DIR/qrcode_from_file.png decode" \
    0 "(qrcode) Testing data from file." ""

# DE-04: Detailed JSON Output
run_test "DE-04" "Decoding with detailed JSON output" \
    "$CLI_PATH $TEST_DIR/qrcode.png decode --detailed-results-json" \
    0 '"format": "QR_CODE"' ""

# DE-05: Raw Bytes Output
run_test "DE-05" "Decoding with raw bytes output" \
    "$CLI_PATH $TEST_DIR/code128.png decode --raw-bytes" \
    0 "105 12 34 56 78 90 85 106" "" 

# DE-06: Invalid - File Not Found
run_test "DE-06" "Invalid command - file not found" \
    "$CLI_PATH $TEST_DIR/non_existent_file.png decode" \
    1 "Error while attempting to locate barcode" ""

# DE-07: Invalid - No Barcode in Image (using the sample.txt file)
run_test "DE-07" "Invalid decode - no barcode in image" \
    "$CLI_PATH $TEST_DIR/sample.txt decode" \
    1 "Error while attempting to locate barcode" ""

# DE-08: Decode with Specific Barcode Type (Success)
run_test "DE-08" "Decoding with correct specific type" \
    "$CLI_PATH $TEST_DIR/qrcode.png decode --barcode-types QR_CODE" \
    0 "(qrcode) Hello, World!" ""

# DE-09: Decode with Specific Barcode Type (Failure)
run_test "DE-09" "Decoding with incorrect specific type" \
    "$CLI_PATH $TEST_DIR/qrcode.png decode --barcode-types CODE_128" \
    1 "Error while attempting to locate barcode" ""


# --- Final Report ---
teardown

echo -e "\n--- Test Summary ---"
if [ "$tests_passed" -eq "$tests_run" ]; then
    echo -e "${GREEN}✅ All ${tests_run} tests passed!${NC}"
else
    echo -e "${RED}❌ ${tests_passed} out of ${tests_run} tests passed.${NC}"
fi
echo -e "--------------------"

# Exit with a status code indicating overall success or failure
[ "$tests_passed" -eq "$tests_run" ]