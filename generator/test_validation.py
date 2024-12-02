import requests

def test_generator_communication():
    print("\nTesting generator communication:")
    
    # Test getting sequences from another generator
    other_generator = "127.0.0.1"
    other_port = 54321  # Different port for another generator
    
    try:
        # Try to get sequences from other generator
        response = requests.get(f"http://{other_generator}:{other_port}/sequence")
        print("\nGetting sequences from other generator:")
        print("Status:", response.status_code)
        print("Response:", response.json() if response.status_code == 200 else response.text)
    except requests.exceptions.ConnectionError:
        print("Could not connect to other generator")

# Run test
test_generator_communication()