#!/usr/bin/env pwsh
# tests/test_llm_backends.ps1
# PowerShell script to test the LLM backend functionality in shell-assistant

Write-Host "Shell Assistant LLM Backend Test Script" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Make sure we're in the right directory
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$rootDir = Split-Path -Parent $scriptDir
Set-Location $rootDir

# Build the application first
Write-Host "Building the application..." -ForegroundColor Yellow
cargo build
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build the application. Exiting." -ForegroundColor Red
    exit 1
}
Write-Host "Build successful!" -ForegroundColor Green
Write-Host ""

# Paths
$cliPath = Join-Path $rootDir "target\debug\cli.exe"

# Test natural language prompts
$prompts = @(
    "list all files in the current directory",
    "get the current date and time",
    "show system information"
)

# Test 1: Default Ollama backend
Write-Host "Test 1: Default Ollama Backend" -ForegroundColor Magenta
Write-Host "-----------------------------" -ForegroundColor Magenta
Write-Host "Testing with prompt: '$($prompts[0])'"
Write-Host "Command: $cliPath --dry-run `"$($prompts[0])`""
Write-Host ""
& $cliPath --dry-run $prompts[0]
Write-Host ""
Write-Host "Default backend test completed." -ForegroundColor Green
Write-Host ""

# Test 2: Explicit Ollama backend
Write-Host "Test 2: Explicit Ollama Backend" -ForegroundColor Magenta
Write-Host "-----------------------------" -ForegroundColor Magenta
Write-Host "Testing with prompt: '$($prompts[1])'"
Write-Host "Command: $cliPath --backend ollama --dry-run `"$($prompts[1])`""
Write-Host ""
& $cliPath --backend ollama --dry-run $prompts[1]
Write-Host ""
Write-Host "Explicit Ollama backend test completed." -ForegroundColor Green
Write-Host ""

# Test 3: Online mode (wizardcoder model)
Write-Host "Test 3: Online Mode (wizardcoder model)" -ForegroundColor Magenta
Write-Host "------------------------------------" -ForegroundColor Magenta
Write-Host "Testing with prompt: '$($prompts[2])'"
Write-Host "Command: $cliPath --online --dry-run `"$($prompts[2])`""
Write-Host ""
& $cliPath --online --dry-run $prompts[2]
Write-Host ""
Write-Host "Online mode test completed." -ForegroundColor Green
Write-Host ""

# Test 4: llm-rs backend
Write-Host "Test 4: llm-rs Backend" -ForegroundColor Magenta
Write-Host "-------------------" -ForegroundColor Magenta
Write-Host "Note: This will fail if the llm-rs feature is not enabled, but will NOT try to use OpenAI as fallback."
Write-Host "Testing with prompt: '$($prompts[0])'"
Write-Host "Command: $cliPath --backend llm-rs --dry-run `"$($prompts[0])`""
Write-Host ""
& $cliPath --backend llm-rs --dry-run $prompts[0]
Write-Host ""
Write-Host "llm-rs backend test completed." -ForegroundColor Green
Write-Host ""

# Test 5: OpenAI backend
Write-Host "Test 5: OpenAI Backend (COMMENTED OUT)" -ForegroundColor Magenta
Write-Host "-------------------" -ForegroundColor Magenta
Write-Host "This test is commented out as OpenAI integration will be implemented later."
Write-Host ""
# Write-Host "Note: This will fail if OPENAI_API_KEY environment variable is not set."
# Write-Host "Testing with prompt: '$($prompts[1])'"
# Write-Host "Command: $cliPath --backend openai --dry-run `"$($prompts[1])`""
# Write-Host ""
# & $cliPath --backend openai --dry-run $prompts[1]
# Write-Host ""
Write-Host "OpenAI backend test skipped." -ForegroundColor Yellow
Write-Host ""

# Test 6: Simulating fallback to OpenAI (intentionally using a non-existent Ollama model)
Write-Host "Test 6: Fallback Test (COMMENTED OUT)" -ForegroundColor Magenta
Write-Host "-----------------------------------" -ForegroundColor Magenta
Write-Host "This test is commented out as OpenAI integration will be implemented later."
Write-Host ""
# Write-Host "Note: This test requires OPENAI_API_KEY environment variable to be set."
# Write-Host "Modifying llm.rs temporarily to force fallback..."

# Create a simple prompt to test fallback
# $fallbackPrompt = "translate 'hello' to spanish"
# Write-Host "Testing with prompt: '$fallbackPrompt'"
# Write-Host "Command: $cliPath --backend ollama --online --dry-run `"$fallbackPrompt`""
# Write-Host ""
# & $cliPath --backend ollama --online --dry-run $fallbackPrompt
# Write-Host ""
Write-Host "Fallback test skipped." -ForegroundColor Yellow
Write-Host ""

Write-Host "All tests completed!" -ForegroundColor Cyan
Write-Host ""
Write-Host "Summary:" -ForegroundColor Yellow
Write-Host "- Test 1: Default Ollama Backend"
Write-Host "- Test 2: Explicit Ollama Backend"
Write-Host "- Test 3: Online Mode (wizardcoder model)"
Write-Host "- Test 4: llm-rs Backend (may fail if feature not enabled)"
Write-Host "- Test 5: OpenAI Backend (SKIPPED - future implementation)"
Write-Host "- Test 6: Fallback Test (SKIPPED - future implementation)"
Write-Host ""
Write-Host "Check the outputs above to verify each backend's behavior." -ForegroundColor Yellow
