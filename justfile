set dotenv-filename := ".env.local"
set dotenv-load

# Application
project := "advent-of-code"

# Configuration
lang := "go"
year := "2024"
day := "00"
part := "1"


# Install required tooling
install:
  @echo "Installing required tooling..."
  @./scripts/install_esb.sh

# Create boilerplate and fetch input data
start:
  @if [ "{{ day }}" = "00" ]; then echo "Please provide a valid day"; exit 1; fi
  @echo "Scaffolding boilerplate for day {{ day }}..."
  @esb fetch --year {{ year }} --day {{ day }}
  @esb start --lang {{ lang }} --year {{ year }} --day {{ day }}
  @cd ./solutions/go/{{ year }}/{{ day }} && go mod tidy

# Test the current solution for a given day
test:
  @if [ "{{ day }}" = "00" ]; then echo "Please provide a valid day"; exit 1; fi
  @echo "Running tests for day {{ day }} part {{ part }}..."
  @esb test --lang {{ lang }} --year {{ year }} --day {{ day }} --part {{ part }}

# Solve the puzzle for a given day
solve:
  @if [ "{{ day }}" = "00" ]; then echo "Please provide a valid day"; exit 1; fi
  @echo "Solving puzzle for day {{ day }} part {{ part }}..."
  @esb run --lang {{ lang }} --year {{ year }} --day {{ day }} --part {{ part }}

# Submit the solution for a given day
submit:
  @if [ "{{ day }}" = "00" ]; then echo "Please provide a valid day"; exit 1; fi
  @echo "Submitting solution for day {{ day }} part {{ part }}..."
  @esb run --lang {{ lang }} --year {{ year }} --day {{ day }} --part {{ part }} --submit

# Fetch puzzle information for a given day
fetch:
  @if [ "{{ day }}" = "00" ]; then echo "Please provide a valid day"; exit 1; fi
  @echo "Fetching puzzle info for day {{ day }}..."
  @esb fetch --year {{ year }} --day {{ day }}

# Check the status of all puzzles
status:
  @echo "Checking the status of all puzzles..."
  @esb status

# Display the problem statement for a given day
show:
  @if [ "{{ day }}" = "00" ]; then echo "Please provide a valid day"; exit 1; fi
  @echo "Displaying the problem statement for day {{ day }}..."
  @esb show --year {{ year }} --day {{ day }}

