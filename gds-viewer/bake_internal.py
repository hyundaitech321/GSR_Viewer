#!/usr/bin/env python3
r"""Bake status_lookup.json into a copy of the viewer -> one internal-only HTML file.
Usage: python bake_internal.py GSR-Viewer.html status_lookup.json GSR-Viewer_0.34.html 0.34
The output contains Hyundai text. Keep it inside HMA; do not publish it."""
import sys, gzip, base64, json
src, lk, out = sys.argv[1:4]
ver = sys.argv[4] if len(sys.argv) > 4 else ''
j = json.load(open(lk, encoding='utf-8'))
assert j.get('v') == 1 and j.get('ecu') and j.get('tables')
blob = base64.b64encode(gzip.compress(json.dumps(j, ensure_ascii=False, separators=(',', ':')).encode('utf-8'), 9)).decode()
s = open(src, encoding='utf-8').read()
assert 'id="bakedLookup"' not in s
BANNER = ('<style>#btnLk,#btnLkClear{display:none!important}'
          '#betaBanner{flex:none;background:#b42318;color:#fff;font:600 13px/1.4 system-ui,Segoe UI,sans-serif;text-align:center;padding:6px 12px;letter-spacing:.02em;position:relative;z-index:1000}</style>\n'
          '<div id="betaBanner" role="note">BETA' + (' v' + ver if ver else '') + ' &mdash; INTERNAL USE ONLY &mdash; DO NOT SHARE. Keep inside the HMA network.</div>\n')
tag = BANNER + '<script id="bakedLookup" type="application/octet-stream">' + blob + '</script>\n'
import re
m = re.search(r'<body[^>]*>', s)
assert m, 'no <body> tag'
s = s[:m.end()] + '\n' + tag + s[m.end():]
open(out, 'w', encoding='utf-8').write(s)
print('wrote', out, len(s), 'bytes;', len(j['ecu']), 'modules')
