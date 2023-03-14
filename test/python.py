import socket
import bson

# create a socket object
server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# get local machine name


# bind the socket to a public host, and a port
server_socket.bind(("localhost", 2121))

# queue up to 5 requests
server_socket.listen(5)

while True:
    # establish a connection
    client_socket, addr = server_socket.accept()
    print("Got a connection from %s" % str(addr))

    # receive the data sent by the Rust client
    data = b''
    while True:
        chunk = client_socket.recv(4096)
        if not chunk:
            break
        data += chunk

    # decode the BSON data into a Python dictionary
    decoded_data = bson.loads(data)

    # print the received data
    print(decoded_data)

    # close the client socket
    client_socket.close()
