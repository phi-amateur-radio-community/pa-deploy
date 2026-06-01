# Pa Deploy

> Author: St Rangeset
> Date: 2026-05-31

## Description

This project is used for the automated deployment of the project on the server, allowing the build process to be decoupled from execution and deployment.

- Supports multiple **Security** options for communication (e.g., HMAC-SHA256 signature, request body encryption).
- **Retrieve deployment status and logs** asynchronously after execution.
- Support GitHub Actions, and provide the [action.yml](/pad-tools/actions/action.yml).

## Quick User Manual

Download to your server from the link [below](#download) and enable it.
And to modify the configuration.

> [!TIP]
> When you can't use the HTTPS, We recommond that you use `AES256-GCM` and `Ed25519` to ensure security.

## Download

You can download from [GitHub Release](https://github.com/phi-amateur-radio-community/pa-deploy/releases).
Alternatively, check [our repositories](https://repos.phiarc.org/index.html) for system-specific packages.

## Document

Related documents and materials are under the [docs](/docs)

## Copyright

If there is no other explanation,
the text content of this project adopts the [CC-BY-SA 4.0 Unported](https://creativecommons.org/licenses/by-sa/4.0/),
and the source code is adopts the [GPLv3 or later](/LICENSE)

> Copyright (c) 2026 Phiarc Team and St Rangeset.  
> This document adopts the [Creative Commons Attribution-ShareAlike 4.0 Unported](https://creativecommons.org/licenses/by-sa/4.0/).
