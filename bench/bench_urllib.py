from urllib.request import urlopen

def get(url):
    return urlopen(url).read()

for i in range(10000):
    get('http://localhost:8080/')

