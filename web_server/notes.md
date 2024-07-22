http requests have the following format where CRLF is carriage return and line fieed
Method Request-URI HTTP-Version CRLF -> GET / HTTP/1.1
headers CRLF -> Host onwards
message-body -> Get requests have no body

response format
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body