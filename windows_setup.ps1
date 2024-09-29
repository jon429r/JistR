
# Set project name (update if necessary)
$projectName = "jist"

# Build the Rust project
Write-Host "Building the project..."
cargo build --release

# Check if build succeeded
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed. Exiting..."
    exit 1
}

# Set the directory to move the binary to (modify as needed)
$binDir = "$env:USERPROFILE\AppData\Local\bin"

# Create bin directory if it doesn't exist
if (-not (Test-Path $binDir)) {
    New-Item -ItemType Directory -Path $binDir
}

# Move the binary to the bin directory
Move-Item "target\release\$projectName.exe" "$binDir\$projectName.exe" -Force

# Add binDir to system PATH if not already there
if (-not ($env:Path -split ";" | ForEach-Object { $_.Trim() } | Where-Object { $_ -eq $binDir })) {
    [System.Environment]::SetEnvironmentVariable("Path", "$env:Path;$binDir", [System.EnvironmentVariableTarget]::User)
    Write-Host "Added $binDir to the system PATH."
} else {
    Write-Host "$binDir is already in the system PATH."
}

# Verify if the binary was successfully moved
if (Test-Path "$binDir\$projectName.exe") {
    Write-Host "Binary successfully installed in $binDir."
    Write-Host "You can now run the compiler from anywhere with the command: $projectName <path_to_file.jist>"
} else {
    Write-Host "Failed to install the binary. Please check your setup and try again."
    exit 1
}
