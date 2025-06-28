# Copy-Executable.ps1
# Copies the Shell Assistant executable to the module directory

# Get current script directory
$ModuleDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ModuleBinDir = Join-Path $ModuleDir "bin"

# Create bin directory if it doesn't exist
if (-Not (Test-Path $ModuleBinDir)) {
    New-Item -Path $ModuleBinDir -ItemType Directory -Force | Out-Null
}

# Paths to look for the executable
$PossiblePaths = @(
    (Join-Path (Split-Path -Parent $ModuleDir) "target\release\cli.exe"),
    (Join-Path (Split-Path -Parent $ModuleDir) "target\debug\cli.exe")
)

$ExecutableFound = $false
foreach ($Path in $PossiblePaths) {
    if (Test-Path $Path) {
        Write-Host "Found executable at: $Path"
        Copy-Item -Path $Path -Destination (Join-Path $ModuleBinDir "shell-assistant.exe") -Force
        $ExecutableFound = $true
        break
    }
}

if ($ExecutableFound) {
    Write-Host "Executable copied to module bin directory: $ModuleBinDir" -ForegroundColor Green
    Write-Host "Module is ready to use!" -ForegroundColor Green
} else {
    Write-Host "Could not find Shell Assistant executable. Please build it with 'cargo build --release' first." -ForegroundColor Red
}
