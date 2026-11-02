"""Read-only TCL firmware metadata query using non-unique platform information.

Protocol established from this TV's TTVSCore.apk. This deliberately never reads
the TV's unique serial, MAC, account token or other private identifiers. No TV
connection, download/install action or test-server query is performed.
"""
import hashlib
import os
import json
from pathlib import Path
import time
import urllib.error
import urllib.request
import uuid

host = "https://na-filter-upgrade.huan.tv"
path = "/service/upmp/upgradeIncrInterface"
body = '''<?xml version="1.0" encoding="UTF-8"?>
<upgradeIncrRequest><apiversion>1.0</apiversion><upmptype>3</upmptype>
<app><appid>V8T221T04</appid><increment>0</increment><ver>V8-T221T04-LF1V109</ver><verid>0</verid></app>
<parameter><callid>0</callid><client><devmodel>TCL-NA-MT9221-02</devmodel><didtoken>00000000000000000000000000000000</didtoken><dnum>000000000</dnum><projectid>17849</projectid><systemver>11</systemver></client><https>true</https><language>en</language><region>US</region><timezone>America/Los_Angeles</timezone></parameter>
</upgradeIncrRequest>'''
request_id = uuid.uuid4().hex
timestamp = str(int(time.time()))
# Supply only a key you are authorized to use; the vendor constant is not distributed.
app_key = os.environ.get("TCL_OTA_APP_KEY")
if not app_key:
    raise SystemExit("Set TCL_OTA_APP_KEY to an authorized request-signing key")
signature = hashlib.md5((app_key + request_id + timestamp + "POST" + path + body).encode()).hexdigest()
headers = {
    "Content-Type": "application/xml; charset=UTF-8",
    "HUAN-AppId": "upmp-na",
    "HUAN-RequestId": request_id,
    "HUAN-Timestamp": timestamp,
    "HUAN-Sign": signature,
}
request = urllib.request.Request(host + path, data=body.encode(), headers=headers, method="POST")
result = {"url": host + path, "request": body, "dummy_identifiers": True}
try:
    with urllib.request.urlopen(request, timeout=20) as response:
        result.update(status=response.status, response=response.read(256 * 1024).decode("utf-8", "replace"))
except urllib.error.HTTPError as error:
    result.update(status=error.code, response=error.read(256 * 1024).decode("utf-8", "replace"))
except Exception as error:
    result["error"] = str(error)
output = Path(__file__).resolve().parents[1] / 'assets/research/firmware-query-result.json'
output.parent.mkdir(parents=True, exist_ok=True)
output.write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps({k: v for k, v in result.items() if k != "request"}, indent=2))
