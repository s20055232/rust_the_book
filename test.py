import requests
from threading import Thread


def test():
    _ = requests.get("http://127.0.0.1:7878")


ts = []
for i in range(10000):
    t = Thread(target=test)
    t.start()
    ts.append(t)

for i in ts:
    i.join()
