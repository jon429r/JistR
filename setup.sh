#!/bin/bash

PROJECT_NAME="jist"

echo "Building the project..."
cargo build --release

# Check if build succeeded
if [ $? -ne 0 ]; then
  echo "Build failed. Exiting..."
  exit 1
fi

# Determine the target directory for the binary
BIN_DIR="/usr/local/bin"

# Check if BIN_DIR is writable
if [ ! -w "$BIN_DIR" ]; then
  echo "You do not have write permissions for $BIN_DIR. Attempting to use sudo..."
  sudo mv target/release/$PROJECT_NAME $BIN_DIR/
else
  mv target/release/$PROJECT_NAME $BIN_DIR/
fi

# Verify if the binary was successfully moved
if [ $? -eq 0 ]; then
  echo "Binary successfully installed in $BIN_DIR."
  echo "You can now run the compiler from anywhere with the command: $PROJECT_NAME <path_to_file.jist>"
else
  echo "Failed to install the binary. Please check your permissions and try again."
  exit 1
fi
