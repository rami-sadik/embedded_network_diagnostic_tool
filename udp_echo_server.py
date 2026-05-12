import socket

HOST = "127.0.0.1"
PORT = 9001

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.bind((HOST, PORT))

print(f"UDP echo server listening on {HOST}:{PORT}")

while True:
    data, address = sock.recvfrom(1024)
    print(f"Received {data!r} from {address}")
    sock.sendto(b"udp-response", address)
