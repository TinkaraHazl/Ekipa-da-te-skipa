import requests
import json

def list_generators():
    print("\nListing all registered generators:")
    try:
        print("Connecting to registry...")
        response = requests.get("http://127.0.0.1:7878/project", timeout=5)  # Add timeout
        print(f"Response status: {response.status_code}")
        print(f"Response text: {response.text}")
        
        generators = response.json()
        
        if generators:
            print("\nFound generators:")
            for gen in generators:
                print(f"- Name: {gen['name']}")
                print(f"  IP: {gen['ip']}")
                print(f"  Port: {gen['port']}")
                print()
        else:
            print("No generators currently registered")
            
    except requests.exceptions.Timeout:
        print("Request timed out - registry not responding")
    except Exception as e:
        print(f"Error: {e}")

# Run it
list_generators()