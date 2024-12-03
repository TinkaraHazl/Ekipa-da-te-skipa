import requests
def test_mix_sequence():
    print("\nTesting Mix sequence:")
    
    body = {
        "range": {
            "from": 0,
            "to": 30,
            "step": 1,
        },
        "parameters": [3, 2],
        "sequences": [
            {"name": "Constant", "parameters": [10], "sequences": []},
            {"name": "Sum", "parameters": [], "sequences": [
                {"name": "Arithmetic", "parameters": [5, 10], "sequences": []},
                {"name": "Constant", "parameters": [10], "sequences": []},
            ]},
        ]
    }
    
    response = requests.post(
        "http://127.0.0.1:9000/sequence/Mix", 
        json=body
    )
    print(response.json())

test_mix_sequence()