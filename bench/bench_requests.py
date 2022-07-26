from requests import Session

http = Session()
get = http.get

for i in range(10000):
    get('http://localhost:8080/').text