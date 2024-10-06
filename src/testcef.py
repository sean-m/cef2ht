import random

def generate_cef_event(event_id):
    src_ip = f"192.168.1.{random.randint(1, 255)}"
    dst_ip = f"10.0.0.{random.randint(1, 255)}"
    spt = random.randint(1024, 65535)
    dpt = random.randint(1, 65535)
    severity = random.choice([3, 5, 7])
    action = random.choice(["Blocked Connection", "Allowed Connection"])
    
    return f"CEF:0|Security|Firewall|1.0|{event_id}|{action}|{severity}|src={src_ip} dst={dst_ip} spt={spt} dpt={dpt} foo= bar="

# Generate 1000 events
for event_id in range(0, 1000000):
    print(generate_cef_event(event_id))
