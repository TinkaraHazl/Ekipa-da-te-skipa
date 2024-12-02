import requests

# Test Mix of Arithmetic and Constant
body = {
    "range": {"from": 0, "to": 5, "step": 1},
    "parameters": [2, 3],  # Take 2 from first, 3 from second
    "sequences": [
        {
            "name": "Arithmetic",
            "parameters": [1, 2],  # Start at 1, add 2
            "sequences": []
        },
        {
            "name": "Constant",
            "parameters": [10],    # Constant value of 10
            "sequences": []
        }
    ]
}

response = requests.post("http://127.0.0.1:12345/sequence/Mix", json=body)
print("\nTesting Mix sequence:")
print("Status:", response.status_code)
print("Result:", response.json() if response.status_code == 200 else response.text)