#!/usr/bin/env python3
"""Forward TV HTTPS connections through the host without terminating TLS."""
import select
import argparse
import socket
import socketserver

class Tunnel(socketserver.StreamRequestHandler):
    def handle(self):
        self.connection.settimeout(20)
        request=self.rfile.readline(4096).decode('ascii').strip().split()
        if len(request)!=3 or request[0]!='CONNECT': return
        host,port=request[1].rsplit(':',1)
        if port!='443' or not any(host==suffix or host.endswith('.'+suffix)
                for suffix in ('youtube.com','googlevideo.com','ytimg.com','googleapis.com','ggpht.com')):
            self.wfile.write(b'HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n'); return
        for _ in range(64):
            if self.rfile.readline(4096) in (b'\r\n',b'\n',b''): break
        try:
            with socket.create_connection((host,443),timeout=20) as upstream:
                self.wfile.write(b'HTTP/1.1 200 Connection established\r\n\r\n');self.wfile.flush()
                peers=(self.connection,upstream)
                while True:
                    ready,_,_=select.select(peers,[],[],60)
                    if not ready: return
                    for source in ready:
                        data=source.recv(65536)
                        if not data: return
                        (upstream if source is self.connection else self.connection).sendall(data)
        except OSError as error: print(host,type(error).__name__,flush=True)

class Server(socketserver.ThreadingTCPServer):
    allow_reuse_address=True
    daemon_threads=True

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--host',default='192.168.77.1')
    parser.add_argument('--port',type=int,default=8899)
    args=parser.parse_args()
    with Server((args.host,args.port),Tunnel) as server:
        print(f'TV HTTPS tunnel on {args.host}:{args.port}; TLS stays on the TV',flush=True)
        server.serve_forever()
