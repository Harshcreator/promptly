# ShellAssistant.psd1
@{
    # Script module or binary module file associated with this manifest.
    RootModule = 'ShellAssistant.psm1'
    
    # Version number of this module.
    ModuleVersion = '0.1.0'
    
    # ID used to uniquely identify this module
    GUID = '12345678-1234-1234-1234-123456789012'
    
    # Author of this module
    Author = 'Shell Assistant Team'
    
    # Description of the functionality provided by this module
    Description = 'PowerShell module that integrates the Shell Assistant CLI as a plugin for PowerShell terminals'
    
    # Minimum version of the PowerShell engine required by this module
    PowerShellVersion = '5.1'
    
    # Functions to export from this module, for best performance, do not use wildcards and do not delete the entry
    FunctionsToExport = @('Invoke-ShellAssistant', 'Get-ShellAssistantHistory', 'Get-ShellAssistantPlugins')
    
    # Cmdlets to export from this module, for best performance, do not use wildcards and do not delete the entry
    CmdletsToExport = @()
    
    # Variables to export from this module
    VariablesToExport = @()
    
    # Aliases to export from this module, for best performance, do not use wildcards and do not delete the entry
    AliasesToExport = @('sa', 'sa-history', 'sa-plugins')
    
    # Private data to pass to the module specified in RootModule/ModuleToProcess
    PrivateData = @{
        PSData = @{
            # Tags applied to this module. These help with module discovery in online galleries.
            Tags = @('CLI', 'Shell', 'Assistant', 'Terminal', 'PowerShell')
            
            # A URL to the license for this module.
            LicenseUri = ''
            
            # A URL to the main website for this project.
            ProjectUri = ''
            
            # ReleaseNotes of this module
            ReleaseNotes = 'Initial release of Shell Assistant PowerShell Module'
        }
    }
}
