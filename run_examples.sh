#!/bin/bash

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "🚀 OATS Examples Runner"
echo "======================="

# Function to run an example
run_example() {
    echo ""
    echo "📋 Running $1..."
    echo "----------------------------------------"
    cd "$SCRIPT_DIR/examples/$1" && cargo run && cd "$SCRIPT_DIR"
    echo "----------------------------------------"
}

# Function to run tests
run_tests() {
    echo ""
    echo "🧪 Running tests..."
    echo "----------------------------------------"
    cd "$SCRIPT_DIR" && cargo test
    echo "----------------------------------------"
}

# Function to run benchmarks
run_benchmarks() {
    echo ""
    echo "⚡ Running benchmarks..."
    echo "----------------------------------------"
    cd "$SCRIPT_DIR" && cargo bench
    echo "----------------------------------------"
}

# Function to run test example
run_test_example() {
    echo ""
    echo "🧪 Running test example..."
    echo "----------------------------------------"
    cd "$SCRIPT_DIR" && cargo run --bin test_example
    echo "----------------------------------------"
}

# Check if specific example is requested
if [ $# -eq 1 ]; then
    case $1 in
        "basic")
            run_example "basic"
            ;;
        "game")
            run_example "game"
            ;;
        "business")
            run_example "business"
            ;;
        "test")
            run_tests
            ;;
        "test-example")
            run_test_example
            ;;
        "bench")
            run_benchmarks
            ;;
        "all")
            run_example "basic"
            run_example "game"
            run_example "business"
            run_test_example
            run_tests
            ;;
        *)
            echo "Usage: $0 [basic|game|business|test|test-example|bench|all]"
            echo "  basic        - Run basic example"
            echo "  game         - Run game example"
            echo "  business     - Run business example"
            echo "  test         - Run all tests"
            echo "  test-example - Run test example binary"
            echo "  bench        - Run benchmarks"
            echo "  all          - Run all examples and tests"
            exit 1
            ;;
    esac
else
    echo "Usage: $0 [basic|game|business|test|test-example|bench|all]"
    echo ""
    echo "Examples:"
    echo "  $0 basic        # Run basic example"
    echo "  $0 game         # Run game example"
    echo "  $0 business     # Run business example"
    echo "  $0 test         # Run all tests"
    echo "  $0 test-example # Run test example binary"
    echo "  $0 bench        # Run benchmarks"
    echo "  $0 all          # Run everything"
fi 