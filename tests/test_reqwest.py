from reqwest import Client

c = Client()
get = c.get
for i in range(100):
    get("http://localhost:8080/")