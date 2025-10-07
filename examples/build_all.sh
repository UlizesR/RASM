#!/bin/bash
# Build all RASM examples

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the RASM binary path
RASM="$(cd "$(dirname "$0")/.." && pwd)/target/release/rasm"

if [ ! -f "$RASM" ]; then
    echo -e "${RED}Error: RASM binary not found at $RASM${NC}"
    echo "Build it first with: cargo build --release"
    exit 1
fi

echo -e "${BLUE}Building all RASM examples...${NC}\n"

# Track results
TOTAL=0
SUCCESS=0
FAILED=0

for dir in */; do
    if [ -d "$dir" ] && [ -f "${dir}.rasm.toml" ]; then
        EXAMPLE_NAME="${dir%/}"
        TOTAL=$((TOTAL + 1))
        
        echo -e "${YELLOW}Building ${EXAMPLE_NAME}...${NC}"
        
        # Extract input files and output from config
        INPUT_FILES=$(grep -E "^input_files" "${dir}.rasm.toml" | sed 's/.*\[\(.*\)\]/\1/' | tr -d '"' | tr ',' ' ')
        OUTPUT_FILE=$(grep -E "^output_file" "${dir}.rasm.toml" | cut -d'"' -f2)
        
        if [ -z "$OUTPUT_FILE" ]; then
            OUTPUT_FILE="$EXAMPLE_NAME"
        fi
        
        if (cd "$dir" && eval "$RASM -o $OUTPUT_FILE $INPUT_FILES" 2>&1); then
            echo -e "${GREEN}✓ ${EXAMPLE_NAME} built successfully${NC}\n"
            SUCCESS=$((SUCCESS + 1))
        else
            echo -e "${RED}✗ ${EXAMPLE_NAME} build failed${NC}\n"
            FAILED=$((FAILED + 1))
        fi
    fi
done

# Summary
echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}Build Summary:${NC}"
echo -e "  Total:   $TOTAL"
echo -e "  ${GREEN}Success: $SUCCESS${NC}"
if [ $FAILED -gt 0 ]; then
    echo -e "  ${RED}Failed:  $FAILED${NC}"
fi
echo -e "${BLUE}================================${NC}"

# Test all examples
if [ $SUCCESS -gt 0 ]; then
    echo -e "\n${BLUE}Testing examples...${NC}\n"
    
    for dir in */; do
        if [ -d "$dir" ] && [ -f "${dir}.rasm.toml" ]; then
            EXAMPLE_NAME="${dir%/}"
            
            # Get output file from .rasm.toml or use directory name
            OUTPUT_FILE=$(grep -E "^output_file" "${dir}.rasm.toml" | cut -d'"' -f2 | cut -d"'" -f2)
            OUTPUT_FILE="${OUTPUT_FILE:-$EXAMPLE_NAME}"
            
            if [ -x "${dir}${OUTPUT_FILE}" ]; then
                echo -e "${YELLOW}Running ${EXAMPLE_NAME}:${NC}"
                (cd "$dir" && "./${OUTPUT_FILE}" && echo -e "${GREEN}✓ Success${NC}\n") || \
                    echo -e "${RED}✗ Failed${NC}\n"
            fi
        fi
    done
fi

exit 0

