#!/usr/bin/env bash
# tests/test_llm_backends.sh
# Bash script to test the LLM backend functionality in shell-assistant

echo -e "\e[36mShell Assistant LLM Backend Test Script\e[0m"
echo -e "\e[36m========================================\e[0m"
echo ""

# Make sure we're in the right directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$ROOT_DIR"

# Build the application first
echo -e "\e[33mBuilding the application...\e[0m"
cargo build
if [ $? -ne 0 ]; then
    echo -e "\e[31mFailed to build the application. Exiting.\e[0m"
    exit 1
fi
echo -e "\e[32mBuild successful!\e[0m"
echo ""

# Paths
CLI_PATH="$ROOT_DIR/target/debug/cli"

# Test natural language prompts
PROMPTS=(
    "list all files in the current directory"
    "get the current date and time"
    "show system information"
)

# Test 1: Default Ollama backend
echo -e "\e[35mTest 1: Default Ollama Backend\e[0m"
echo -e "\e[35m-----------------------------\e[0m"
echo "Testing with prompt: '${PROMPTS[0]}'"
echo "Command: $CLI_PATH --dry-run \"${PROMPTS[0]}\""
echo ""
"$CLI_PATH" --dry-run "${PROMPTS[0]}"
echo ""
echo -e "\e[32mDefault backend test completed.\e[0m"
echo ""

# Test 2: Explicit Ollama backend
echo -e "\e[35mTest 2: Explicit Ollama Backend\e[0m"
echo -e "\e[35m-----------------------------\e[0m"
echo "Testing with prompt: '${PROMPTS[1]}'"
echo "Command: $CLI_PATH --backend ollama --dry-run \"${PROMPTS[1]}\""
echo ""
"$CLI_PATH" --backend ollama --dry-run "${PROMPTS[1]}"
echo ""
echo -e "\e[32mExplicit Ollama backend test completed.\e[0m"
echo ""

# Test 3: Online mode (wizardcoder model)
echo -e "\e[35mTest 3: Online Mode (wizardcoder model)\e[0m"
echo -e "\e[35m------------------------------------\e[0m"
echo "Testing with prompt: '${PROMPTS[2]}'"
echo "Command: $CLI_PATH --online --dry-run \"${PROMPTS[2]}\""
echo ""
"$CLI_PATH" --online --dry-run "${PROMPTS[2]}"
echo ""
echo -e "\e[32mOnline mode test completed.\e[0m"
echo ""

# Test 4: llm-rs backend
echo -e "\e[35mTest 4: llm-rs Backend\e[0m"
echo -e "\e[35m-------------------\e[0m"
echo "Note: This will fail if the llm-rs feature is not enabled, but will NOT try to use OpenAI as fallback."
echo "Testing with prompt: '${PROMPTS[0]}'"
echo "Command: $CLI_PATH --backend llm-rs --dry-run \"${PROMPTS[0]}\""
echo ""
"$CLI_PATH" --backend llm-rs --dry-run "${PROMPTS[0]}"
echo ""
echo -e "\e[32mllm-rs backend test completed.\e[0m"
echo ""

# Test 5: OpenAI backend
echo -e "\e[35mTest 5: OpenAI Backend (COMMENTED OUT)\e[0m"
echo -e "\e[35m-------------------\e[0m"
echo "This test is commented out as OpenAI integration will be implemented later."
echo ""
# echo "Note: This will fail if OPENAI_API_KEY environment variable is not set."
# echo "Testing with prompt: '${PROMPTS[1]}'"
# echo "Command: $CLI_PATH --backend openai --dry-run \"${PROMPTS[1]}\""
# echo ""
# "$CLI_PATH" --backend openai --dry-run "${PROMPTS[1]}"
# echo ""
echo -e "\e[33mOpenAI backend test skipped.\e[0m"
echo ""

# Test 6: Simulating fallback to OpenAI (intentionally using a non-existent Ollama model)
echo -e "\e[35mTest 6: Fallback Test (COMMENTED OUT)\e[0m"
echo -e "\e[35m-----------------------------------\e[0m"
echo "This test is commented out as OpenAI integration will be implemented later."
echo ""
# echo "Note: This test requires OPENAI_API_KEY environment variable to be set."

# Create a simple prompt to test fallback
# FALLBACK_PROMPT="translate 'hello' to spanish"
# echo "Testing with prompt: '$FALLBACK_PROMPT'"
# echo "Command: $CLI_PATH --backend ollama --online --dry-run \"$FALLBACK_PROMPT\""
# echo ""
# "$CLI_PATH" --backend ollama --online --dry-run "$FALLBACK_PROMPT"
# echo ""
echo -e "\e[33mFallback test skipped.\e[0m"
echo ""

echo -e "\e[36mAll tests completed!\e[0m"
echo ""
echo -e "\e[33mSummary:\e[0m"
echo "- Test 1: Default Ollama Backend"
echo "- Test 2: Explicit Ollama Backend" 
echo "- Test 3: Online Mode (wizardcoder model)"
echo "- Test 4: llm-rs Backend (may fail if feature not enabled)"
echo "- Test 5: OpenAI Backend (SKIPPED - future implementation)"
echo "- Test 6: Fallback Test (SKIPPED - future implementation)"
echo ""
echo -e "\e[33mCheck the outputs above to verify each backend's behavior.\e[0m"
