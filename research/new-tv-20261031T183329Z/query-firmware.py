"""Read-only firmware discovery from the G13 stock updater's production service.

No TV command is sent. Unique identifiers are dummy values. Project ID 0 means
unknown; a response obtained with it is a platform candidate, not a proven
package assignment for this particular TV. No download or install is triggered.
"""
import argparse
import hashlib
import os
import json
from pathlib import Path
import re
import time
import urllib.error
import urllib.request
import uuid

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--project', type=int, default=0)
parser.add_argument('--version', default='V8-T221T10-LF1V013')
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
if not re.fullmatch(r'V8-T221T10-LF1V\d+', args.version):
    parser.error('Expected a T221T10 firmware version')
if args.output.exists():
    parser.error('Output already exists; preserve the previous response')
host = 'https://na-filter-upgrade.huan.tv'
path = '/service/upmp/upgradeIncrInterface'
body = f'''<?xml version="1.0" encoding="UTF-8"?>
<upgradeIncrRequest><apiversion>1.0</apiversion><upmptype>3</upmptype>
<app><appid>V8T221T10</appid><increment>0</increment><ver>{args.version}</ver><verid>0</verid></app>
<parameter><callid>0</callid><client><devmodel>TCL-NA-MT9221-03</devmodel><didtoken>00000000000000000000000000000000</didtoken><dnum>000000000</dnum><projectid>{args.project}</projectid><systemver>14</systemver></client><https>true</https><language>en</language><region>US</region><timezone>America/Los_Angeles</timezone></parameter>
</upgradeIncrRequest>'''
request_id = uuid.uuid4().hex
timestamp = str(int(time.time()))
# Supply only a key you are authorized to use; the vendor constant is not distributed.
app_key = os.environ.get("TCL_OTA_APP_KEY")
if not app_key:
    raise SystemExit("Set TCL_OTA_APP_KEY to an authorized request-signing key")
signature = hashlib.md5((app_key + request_id + timestamp + 'POST' + path + body).encode()).hexdigest()
headers = {'Content-Type': 'application/xml; charset=UTF-8',
           'HUAN-AppId': 'upmp-na', 'HUAN-RequestId': request_id,
           'HUAN-Timestamp': timestamp, 'HUAN-Sign': signature}
request = urllib.request.Request(host + path, data=body.encode(), headers=headers, method='POST')
result = {'url': host + path, 'request': body, 'dummy_identifiers': True,
          'project_id_known': args.project != 0, 'queried_at_unix': int(timestamp)}
try:
    with urllib.request.urlopen(request, timeout=25) as response:
        result.update(status=response.status, response=response.read(256 * 1024).decode('utf-8', 'replace'))
except urllib.error.HTTPError as error:
    result.update(status=error.code, response=error.read(256 * 1024).decode('utf-8', 'replace'))
except Exception as error:
    result['error'] = str(error)
args.output.parent.mkdir(parents=True, exist_ok=True)
args.output.write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({k: v for k, v in result.items() if k != 'request'}, indent=2))
