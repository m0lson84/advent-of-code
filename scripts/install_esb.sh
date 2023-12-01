#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# Install the ElfScript Brigade (ESB) CLI.
# ---------------------------------------------------------------------------

#######################################
# Check whether Python is installed on the system
#######################################
function check_python() {
  if ! command -v python &>/dev/null; then
    echo "Python is not installed. Please install Python first."
    exit 1
  fi

  version=$(python --version)
  if [[ $version =~ "Python 2" ]]; then
    echo "Python 2 is not supported. Please install Python 3."
    exit 1
  fi
}

# Check if Python is installed
check_python

echo "Installing esb CLI..."
python -m pip install esb
