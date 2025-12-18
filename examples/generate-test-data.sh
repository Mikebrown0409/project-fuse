#!/bin/bash
# Generate test data for compliance checking

echo "Generating test data files..."

# Generate SOC2 test data with 1000+ events
python3 << 'EOF'
import json
import random
from datetime import datetime, timedelta

# Generate 1000 access log events
events = []
base_time = datetime(2025, 1, 15, 10, 0, 0)
users = [f"user{i}@example.com" for i in range(1, 101)]
resources = [f"document-{i}" for i in range(1, 1001)]
actions = ["read", "write", "delete", "update"]

for i in range(1000):
    events.append({
        "timestamp": (base_time + timedelta(seconds=i*60)).isoformat() + "Z",
        "user": random.choice(users),
        "action": random.choice(actions),
        "resource": random.choice(resources),
        "ip_address": f"192.168.1.{random.randint(1, 254)}"
    })

with open("systems/sample-saas-logs-1000.json", "w") as f:
    json.dump({"access_logs": events}, f, indent=2)

print(f"Generated {len(events)} access log events")
EOF

echo "Test data generation complete!"

