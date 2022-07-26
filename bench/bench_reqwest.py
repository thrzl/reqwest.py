from reqwest import Client

c = Client()
get = c.get
for i in range(10000):
    get("http://localhost:8080/")