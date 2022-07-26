from requests import Session

http = Session()
get = http.get

for i in range(100):
    get('http://localhost:8080/').text