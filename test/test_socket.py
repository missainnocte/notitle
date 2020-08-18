import socket
import time


def get_time():
    return time.asctime(time.localtime(time.time()))


def send_data(data):
    print('tcp send start:\t\t', get_time())
    s.send(data)
    print('tcp send end:\t\t', get_time())


def recv_data():
    print('tcp recv start:\t\t', get_time())
    # 接收数据:
    buffer = []
    # while True:
    # 每次最多接收1k字节:
    # print("recv")
    d = s.recv(1024)
    # print(d)
    # if d:
        # print('-ok')
    buffer.append(d)
    # else:
        # print('eof')
        # break
    data = b''.join(buffer)
    print(data.decode('utf-8'))
    print('tcp recv end:\t\t', get_time())
    return data


s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

print('tcp conn start:\t\t', get_time())
s.connect(("localhost", 3154))
s.setsockopt(socket.SOL_SOCKET, socket.SO_KEEPALIVE, True)
s.ioctl(socket.SIO_KEEPALIVE_VALS, (1, 60 * 1000, 30 * 1000))


send_data(b'this is very good!')
recv_data()
send_data(b'12333')
recv_data()


s.close()
print('tcp conn end:\t\t', get_time())
