#!/bin/bash

# Script to automatically discover and run all Rust examples in the examples directory
# This script will find all .rs files in the examples directory (excluding the current script)
# and run them using cargo run --example

# Thank you AI!

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to print section headers
print_header() {
    echo
    print_status $BLUE "═══════════════════════════════════════════════════════════════"
    print_status $BLUE "$1"
    print_status $BLUE "═══════════════════════════════════════════════════════════════"
    echo
}

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Change to project root directory
cd "$PROJECT_ROOT"

print_header "Axin Examples Runner"
print_status $YELLOW "Project root: $PROJECT_ROOT"
print_status $YELLOW "Examples directory: $SCRIPT_DIR"

# First, build all examples to check for compilation errors
print_header "Building All Examples"
if cargo build --examples --verbose; then
    print_status $GREEN "✓ All examples built successfully"
else
    print_status $RED "✗ Failed to build examples"
    exit 1
fi

# Find all .rs files in the examples directory
EXAMPLE_FILES=($(find "$SCRIPT_DIR" -name "*.rs" -type f | sort))

if [ ${#EXAMPLE_FILES[@]} -eq 0 ]; then
    print_status $RED "No example files found in $SCRIPT_DIR"
    exit 1
fi

print_header "Found ${#EXAMPLE_FILES[@]} Example Files"
for file in "${EXAMPLE_FILES[@]}"; do
    basename_file=$(basename "$file" .rs)
    print_status $YELLOW "  • $basename_file"
done

# Run each example
print_header "Running Examples"

SUCCESS_COUNT=0
TOTAL_COUNT=0

for example_file in "${EXAMPLE_FILES[@]}"; do
    # Get the example name (filename without .rs extension)
    example_name=$(basename "$example_file" .rs)
    
    TOTAL_COUNT=$((TOTAL_COUNT + 1))
    
    print_status $BLUE "Running example: $example_name"
    echo "────────────────────────────────────────────────────────────────"
    
    # Run the example
    if cargo run --example "$example_name"; then
        print_status $GREEN "✓ $example_name completed successfully"
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    else
        print_status $RED "✗ $example_name failed"
    fi
    
    echo
done

# Print summary
print_header "Summary"
if [ $SUCCESS_COUNT -eq $TOTAL_COUNT ]; then
    print_status $GREEN "✓ All $TOTAL_COUNT examples ran successfully!"
    exit 0
else
    FAILED_COUNT=$((TOTAL_COUNT - SUCCESS_COUNT))
    print_status $RED "✗ $FAILED_COUNT out of $TOTAL_COUNT examples failed"
    print_status $YELLOW "  Successful: $SUCCESS_COUNT"
    print_status $YELLOW "  Failed: $FAILED_COUNT"
    exit 1
fi