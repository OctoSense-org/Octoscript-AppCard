import re
"""Serial HTTP access to one persistent, version-matched Studio remote client."""
import argparse,json,queue,subprocess,threading,time
from pathlib import Path
from http.server import BaseHTTPRequestHandler,HTTPServer


def query_batch(requests, process, messages, timeout=30):
    """Pipeline exact queries; never accept an unrelated or duplicate reply."""
    expected = set()
    if not isinstance(requests, list) or not 1 <= len(requests) <= 128:
        raise ValueError('WidgetQuery batches require 1–128 requests')
    for request in requests:
        if set(request) != {'WidgetQuery'}:
            raise ValueError('only WidgetQuery can be batched')
        value = request['WidgetQuery']
        key = (json.dumps(value['build_id']), value['query'])
        if not value['query'].startswith('id:') or key in expected:
            raise ValueError('batch queries must have distinct exact widget IDs')
        expected.add(key)
    process.stdin.write(''.join(json.dumps(request)+'\n' for request in requests))
    process.stdin.flush()
    replies = []
    deadline = time.monotonic() + min(60, timeout)
    while expected:
        row = messages.get(timeout=max(.01, deadline-time.monotonic()))
        if 'Error' in row:
            raise RuntimeError(str(row['Error']))
        if 'WidgetQuery' not in row:
            continue
        reply = row['WidgetQuery']
        key = (json.dumps(reply['build_id']), reply['query'])
        if key not in expected:
            raise RuntimeError('unexpected or duplicate WidgetQuery reply')
        expected.remove(key)
        replies.append(reply)
    return {'responses': replies}


def main():
    p=argparse.ArgumentParser();p.add_argument('--binary',required=True);p.add_argument('--studio',default='127.0.0.1:8001')
    p.add_argument('--port',type=int,default=8168);p.add_argument('--log',required=True);a=p.parse_args()
    log=Path(a.log);log.parent.mkdir(parents=True,exist_ok=True);output=log.open('a')
    process=subprocess.Popen([a.binary,'studio','--studio='+a.studio],stdin=subprocess.PIPE,stdout=subprocess.PIPE,
                             stderr=subprocess.STDOUT,text=True,bufsize=1)
    messages=queue.Queue()
    def read():
        for line in process.stdout:
            output.write(line);output.flush()
            try:messages.put(loads_row(line))
            except ValueError:pass
        messages.put({'Error':{'message':'Studio bridge process ended'}})
    threading.Thread(target=read,daemon=True).start()
    class Handler(BaseHTTPRequestHandler):
        def reply(self,value,status=200):
            raw=json.dumps(value).encode();self.send_response(status);self.send_header('Content-Type','application/json')
            self.end_headers();self.wfile.write(raw)
        def do_GET(self):self.reply({'studio':a.studio,'binary':a.binary,'running':process.poll() is None,'batch_widget_queries':True})
        def do_POST(self):
            try:
                data=json.loads(self.rfile.read(int(self.headers['Content-Length'])))
                if process.poll() is not None:raise RuntimeError('persistent bridge is not running')
                while not messages.empty():messages.get_nowait()
                if 'requests' in data:
                    self.reply(query_batch(data['requests'],process,messages,data.get('timeout',30)))
                    return
                process.stdin.write(json.dumps(data['request'])+'\n');process.stdin.flush()
                kind=data.get('response');result={'sent':True};deadline=time.monotonic()+min(60,data.get('timeout',30))
                if kind:
                    while True:
                        row=messages.get(timeout=max(.01,deadline-time.monotonic()))
                        if 'Error' in row:raise RuntimeError(str(row['Error']))
                        if kind in row:
                            if kind=='QueryLogResults' and not row[kind].get('done'):continue
                            result=row[kind];break
                self.reply(result)
            except (ValueError,KeyError,RuntimeError,queue.Empty,OSError) as error:self.reply({'error':str(error) or 'Studio response timed out'})
        def log_message(self,*args):pass
    server=HTTPServer(('127.0.0.1',a.port),Handler)
    print(f'Persistent Studio bridge: http://127.0.0.1:{a.port}, Studio {a.studio}',flush=True)
    try:server.serve_forever()
    finally:server.server_close();process.terminate();process.wait(timeout=5);output.close()


def loads_row(line):
    """A hub row as JSON. The bridge's serializer leaves a comma before the
    closing brace when a record's trailing optional fields are absent (every
    WidgetSnapshot widget without text does this), which strict JSON rejects;
    dropping such rows made every snapshot wait time out."""
    try:
        return json.loads(line)
    except ValueError:
        return json.loads(re.sub(r',\s*([}\]])', r'\1', line))


if __name__=='__main__':main()
