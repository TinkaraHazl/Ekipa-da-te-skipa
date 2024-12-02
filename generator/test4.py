import requests

# Test ping directly
print("Testing ping directly:")
response = requests.get("http://127.0.0.1:12345/ping/")
print("Ping status:", response.status_code)
print("Ping data:", response.json() if response.status_code == 200 else response.text)

# Test sequence endpoint
print("\nTesting sequence endpoint:")
response = requests.get("http://127.0.0.1:12345/sequence")
print("Sequence status:", response.status_code)
print("Available sequences:", response.json() if response.status_code == 200 else response.text)