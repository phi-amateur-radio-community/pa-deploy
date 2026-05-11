# Pa Deploy API Document

> Author: St Rangeset
> Data: 2026-05-11

## Request

**METHOD: `POST`**

### Header Parameter

#### Signature Header

If you don't want to enable signature verification, please [skip it](#other-header).

If you want to enable it, this part of header is required, and please read [Siguature](#signature).

> This is a n in-page link.
> If you can't jump, it's at the end of the document.

```
X-Signature: <Signature>
X-Timestamp: <Timestamp>
X-Nonce: <Nonce>
```

#### Other header

- `Content-Type: application/json `
  > **required**

  Only `application/json`.

***

- `Authorization: Bearer <Token>`
  > **recommand**

  **Paremeter:**

  - Token: Access token set by your server

  If the server enables authentication of Token, this must be filled in as your token.

  **!!! Warning: If SSL/TLS isn't enabled, your token may be leaked !!!**

***

- `Encrypted: { none (default) | aes256 | rsa }`

  If you want to encrypt the body, you must set it.

### Body Parameter

- `wait_status: { false (default) | true }`

  If you want to wait for the running status of the server, you can set it to true.

  If it is true, the server will wait for a moment (configure at server) and carry the log (after tar) in the response body, and include `X-Run-Status: { Running | Succeeded | Failed }` at header.

  > If you server without configuration about mark of status, the Run-Status will disappear

  And you can used `GET` and include the `Session-Code: <session_code>` `Authorization: Bearer <Token>` (if you enabled it) and other contents the same as the `POST` request at header, the server will send the running log and `X-Run-Status` again.

- `session_code: <session_code>`
  > **required**

  A unique value is bound to a single deployment process.
  It determines the path of log file, and the some others.

  We recommend that you use the hash code of commit.


## Response

- `20x`

  It marks the success of the request.

  - `200 Ok`

    When you enable `wait_status` and its value isn't `Running`.

    > **Header**
    - `Content-Type: application/x-tar`
    - `X-Run-Status: { Succeeed | Failed }`

      Status of this deployment.

    > **Body:** Log files packaged by the tar, includes `stdout` and `stderr`

  - `202 Accepted`

    When you enable `wait_status` and its value is `Running`.

    > **Header**
    - `Content-Type: application/x-tar`
    - `X-Run-Status: Running`

      Status of this deployment.

    > **Body:** Log files packaged by the tar , includes `stdout` and `stderr`

  - `204 No Content`

    When you disable `wait_status`.

    **Null Content**

- `40x`

  Please check the standard status code of the HTTP.
  If it is an authentication error, the `Content-Type` will be set to `text/plain` and include the error message in the body.

  If the signature does not match, you can find the log files on the server, or use the process mode `-P` of the client to analyze your signature process.
  Of course, you can also use the client of this software.

- `50x`

  Please check the standard status code of the HTTP.
  If you think it's a server problem, you can check the server or ask the server administrator.

## Signature

### Header Parameter

- `X-Signature` The signature calculated in the following text. Don't fill in unsigned request.
- `X-Timestamp` The timestamp when sending the request, which is blocked according to the threshold set by the server, to prevent replay attacks.
- `X-Nonce` Random number, which is also used to prevent replay attacks. After the server is stored for a moment (at configuaration of server), the server will to block the requests with the same random number.

### Build an Unsigned Request

**HTTP Request Format**

```
<METHOD>(Space)<URI_PATH>(Space)<HTTP_VERSION>
[ Header ]
(Blank Line)
[ Body ]
```

- `METHOD`: Request method.

- `URI_PATH`: The path of the uri.  
  **e.g.**  
  > URI: `https://example.com:443/aaa/bbb?xxx=yyy&mmm=nnn#title`  
  > The `https` is scheme, `example.com` is host, `443` is port (in http, if it is `80`, usually don't need to fill. Similarly, it defaults to 443 in HTTPS), `aaa/bbb` is path, `xxx-yyy` and `mmm=nnn` is query, `title` is fragment.  
  > For details, please refer to [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986).

- `HTTP_VERSION`: The version of the HTTP protocol you are using. (e.g., `HTTP/1.1`, `HTTP/2`).

- `Header`: Key and value list of header, and arrange in alphabetical order.  
  **e.g.**
  ```
  Content-Type: application/json
  Content-Length: xxx
  Encrypted: xxx
  Host: example.com
  X-Nonce: xxx
  X-Timestamp: xxx
  ```
  Must contain `Content-Type` `Host`, and other headers you enable except `X-Signature`.
  If its method is `POST`, it also must to include `Content-Length`.

- `Body`: Is must be consistent with the request body.

### Sign

Use `HMAC-SHA256`, and the `Secret Key` needs to be the same as the server.

You will get a string as the value of the `X-Signature`.

[Return to the header](#signature-header)


> Copyright (c) 2026 Phiarc Team and St Rangeset.  
> This document adopts the [Creative Commons Attribution-ShareAlike 4.0 Unported](https://creativecommons.org/licenses/by-sa/4.0/).