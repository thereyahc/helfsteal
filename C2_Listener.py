import socket, threading,sqlite3
class ClientThread(threading.Thread):
    def __init__(self,clientAddress,clientsocket):
        threading.Thread.__init__(self)
        self.csocket = clientsocket
        print ("New connection added: ", clientAddress)
    def run(self):
        print ("Connection from : ", clientAddress)
        f = open("clientfile","wb")
        while True:
            try:
                data = self.csocket.recv(9999)
                f.write(data)
            except ConnectionResetError:
                print("Connection Failed",clientAddress)		
        print ("Client at ", clientAddress , " disconnected...")
LOCALHOST = "192.168.xx.xx"
PORT = 1234
server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
server.bind((LOCALHOST, PORT))
while True:
    server.listen(1)
    clientsock, clientAddress = server.accept()
    newthread = ClientThread(clientAddress, clientsock)
    newthread.start()    
