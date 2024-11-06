import sys

# Check if a hex string was provided as input
if len(sys.argv) < 2:
    print("Usage: python3 hex_to_uint8_array.py <hex_string>")
    sys.exit(1)

# Get the hex string from the command-line argument
hex_string = sys.argv[1]

# Convert the hex string to a list of uint8 values
uint8_array = [int(hex_string[i:i+2], 16) for i in range(0, len(hex_string), 2)]

# Print the result as a uint8 array
print(uint8_array)
