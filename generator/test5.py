import requests
def test_mix_sequence():
    print("\nTesting Mix sequence:")
    
    body = {
        "range": {"from": 0, "to": 5, "step": 1},
        "parameters": [2, 3],  # Take 2 from first, 3 from second
        "sequences": [
            {
                "name": "Arithmetic",
                "parameters": [1, 2],
                "sequences": []
            },
            {
                "name": "Constant",
                "parameters": [10],
                "sequences": []
            }
        ]
    }
    
    response = requests.post(
        "http://127.0.0.1:12345/sequence/Mix", 
        json=body
    )
    print("Status:", response.status_code)
    print("Result:", response.json() if response.status_code == 200 else response.text)

# Run test
test_mix_sequence()