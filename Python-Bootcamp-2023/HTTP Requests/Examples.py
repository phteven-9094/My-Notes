import requests #Must be installed via pip


# response = requests.get("https://news.ycombinator.com/")

url = "http://www.google.com"
response = requests.get(url)

print(f"your request to {url} came back w/ status code {response.status_code}")