import webbrowser
import threading
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import urlparse, parse_qs
from urllib import request

application_id = "4e5062aee8a96b38521d83f2ab18e81814f99c136dd09a85a17fea6d8b70b23c"
secret = "39185dccb2129f2c78a1349a447ce7f52df2ffee504dd49521d17883b0d827d6"
callback_url = "http://127.0.0.1:4000"
scope = "api"

def main():
    cb = urlparse(callback_url)
    server = ThreadingHTTPServer((cb.hostname, cb.port), get_code)
    server_handler = threading.Thread(target=server.serve_forever)
    server_handler.daemon = True
    server_handler.start()
    request_authorization_code()
    server_handler.join()
    
def request_authorization_code():
    state = "unsafe" # nonce
    url = r"https://gitlab.com/oauth/authorize?client_id=" + application_id + \
            r"&redirect_uri=" + callback_url + r"&response_type=code&state=" + state + r"&scope=" + scope
    webbrowser.open_new_tab(url=url)

class get_code(SimpleHTTPRequestHandler):
    def log_message(self, format, *args):
        pass # suppress logs

    def do_GET(self) -> None:
        query_components = parse_qs(urlparse(self.path).query)
        code = query_components.get("code")
        self.send_response(200)
        self.end_headers()
        if code != None:
            request_token(code[0])

def request_token(code: str):
    url = r"https://gitlab.com/oauth/token?client_id=" + application_id + r"&client_secret=" + secret + \
        r"&code=" + code + r"&grant_type=authorization_code&redirect_uri=" + callback_url
    req = request.Request(url=url, method='POST')
    resp = request.urlopen(req)
    print(resp.read())

if __name__ == "__main__":
    main()