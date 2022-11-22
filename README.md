# tiny-http-server
http server built using tiny-http library (mainly as a debugging/mocking tool).

It will print the request headers and the body.


CLI Parameters (Optional):
1. Port# - default "8888"
2. docroot - default "."
3. extension to content type mapping file - default's built into the binary.

Default Extension to Content Type Mapping (in toml format):
"css" = "text/css"
"gif" = "image/gif"
"htm" = "text/html; charset=utf8"
"html" = "text/html; charset=utf8"
"jpeg" = "image/jpeg"
"jpg" = "image/jpeg"
"js" = "text/javascript"
"json" = "application/json"
"pdf" = "application/pdf"
"png" = "image/png"
"svg" = "image/svg+xml"
"txt" = "text/plain; charset"

unmapped extension will return content-type "application/unknown"

Content Type Mapping file can be in TOML, JSON, YAML, INI formats.