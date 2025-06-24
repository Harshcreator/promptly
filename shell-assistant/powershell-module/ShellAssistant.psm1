# ShellAssistant.psm1
# PowerShell module for integrating Shell Assistant into PowerShell terminal

# Path to the Shell Assistant executable
$ShellAssistantPath = "$PSScriptRoot\bin\shell-assistant.exe"

# Check if the executable exists
if (-Not (Test-Path $ShellAssistantPath)) {
    Write-Error "Shell Assistant executable not found at: $ShellAssistantPath"
    return
}

function Invoke-ShellAssistant {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true, Position = 0, ValueFromPipeline = $true)]
        [string]$Query,
        
        [Parameter()]
        [switch]$DryRun,
        
        [Parameter()]
        [switch]$Force,
        
        [Parameter()]
        [switch]$DebugMode,
        
        [Parameter()]
        [string]$Plugin,
        
        [Parameter()]
        [string]$Backend = "ollama",
        
        [Parameter()]
        [switch]$Online,
        
        [Parameter()]
        [switch]$Offline,
        
        [Parameter()]
        [string]$ModelPath
    )
      # Build arguments
    $arguments = @()
    
    if ($DryRun) { $arguments += "--dry-run" }
    if ($Force) { $arguments += "--force" }
    if ($DebugMode) { $arguments += "--debug" }
    if ($Plugin) { $arguments += "--plugin", $Plugin }
    if ($Backend) { $arguments += "--backend", $Backend }
    if ($Online) { $arguments += "--online" }
    if ($Offline) { $arguments += "--offline" }
    if ($ModelPath) { $arguments += "--model-path", $ModelPath }
    
    # Add the query at the end
    $arguments += $Query
    
    # Execute Shell Assistant
    & $ShellAssistantPath $arguments
}

function Get-ShellAssistantHistory {
    [CmdletBinding()]
    param()
    
    & $ShellAssistantPath --history
}

function Get-ShellAssistantPlugins {
    [CmdletBinding()]
    param()
    
    & $ShellAssistantPath --list-plugins
}

# Aliases for easier access
Set-Alias -Name sa -Value Invoke-ShellAssistant
Set-Alias -Name sa-history -Value Get-ShellAssistantHistory
Set-Alias -Name sa-plugins -Value Get-ShellAssistantPlugins

# Export functions and aliases
Export-ModuleMember -Function Invoke-ShellAssistant, Get-ShellAssistantHistory, Get-ShellAssistantPlugins
Export-ModuleMember -Alias sa, sa-history, sa-plugins

