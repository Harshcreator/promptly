# Install-ShellAssistant.ps1
# Installation script for Shell Assistant PowerShell Module

# Get the module directory path
$moduleName = "ShellAssistant"
$modulePath = "$env:USERPROFILE\Documents\WindowsPowerShell\Modules\$moduleName"

# Check if module directory exists, create if not
if (-Not (Test-Path $modulePath)) {
    Write-Host "Creating module directory at: $modulePath"
    New-Item -Path $modulePath -ItemType Directory -Force | Out-Null
}

# Create bin directory in the module folder
$moduleBinPath = Join-Path $modulePath "bin"
if (-Not (Test-Path $moduleBinPath)) {
    New-Item -Path $moduleBinPath -ItemType Directory -Force | Out-Null
}

# Copy module files
Write-Host "Copying module files..."
Copy-Item -Path "$PSScriptRoot\ShellAssistant.psm1" -Destination $modulePath -Force
Copy-Item -Path "$PSScriptRoot\ShellAssistant.psd1" -Destination $modulePath -Force

# Copy executable
if (Test-Path "$PSScriptRoot\bin\shell-assistant.exe") {
    Write-Host "Copying executable..."
    Copy-Item -Path "$PSScriptRoot\bin\shell-assistant.exe" -Destination "$moduleBinPath\shell-assistant.exe" -Force
} else {
    Write-Host "Executable not found in module bin directory. Checking other locations..." -ForegroundColor Yellow
    $possiblePaths = @(
        (Join-Path (Split-Path -Parent $PSScriptRoot) "target\release\cli.exe"),
        (Join-Path (Split-Path -Parent $PSScriptRoot) "target\debug\cli.exe")
    )
    
    $executableFound = $false
    foreach ($path in $possiblePaths) {
        if (Test-Path $path) {
            Write-Host "Found executable at: $path"
            Copy-Item -Path $path -Destination "$moduleBinPath\shell-assistant.exe" -Force
            $executableFound = $true
            break
        }
    }
    
    if (-Not $executableFound) {
        Write-Host "Could not find Shell Assistant executable. Please build it with 'cargo build --release' first." -ForegroundColor Red
    }
}

# Update module path to point to the copied executable
$moduleContent = Get-Content -Path "$modulePath\ShellAssistant.psm1" -Raw
$moduleContent = $moduleContent -replace '# Path to the Shell Assistant executable[\s\S]*?return\r?\n\}', "# Path to the Shell Assistant executable`r`n`$ShellAssistantPath = `"`$PSScriptRoot\bin\shell-assistant.exe`"`r`n`r`n# Check if the executable exists`r`nif (-Not (Test-Path `$ShellAssistantPath)) {`r`n    Write-Error `"Shell Assistant executable not found at: `$ShellAssistantPath`"`r`n    return`r`n}"
Set-Content -Path "$modulePath\ShellAssistant.psm1" -Value $moduleContent

# Import the module to make sure it works
Write-Host "Importing module..."
Import-Module -Name $moduleName -Force -ErrorAction SilentlyContinue

if (Get-Module -Name $moduleName) {
    Write-Host "Shell Assistant module installed successfully!" -ForegroundColor Green
    Write-Host "You can now use the following commands:" -ForegroundColor Green
    Write-Host "  sa 'your request'         - Process a natural language request" -ForegroundColor Cyan
    Write-Host "  sa-history                - Show command history" -ForegroundColor Cyan
    Write-Host "  sa-plugins                - List available plugins" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "For more options, use: Get-Help Invoke-ShellAssistant" -ForegroundColor Yellow
} else {
    Write-Host "Failed to import module. Please check the installation." -ForegroundColor Red
}
