#!/bin/bash
set -e

DOWNLOAD_URI="https://raw.githubusercontent.com/phi-amateur-radio-community/pa-deploy/refs/heads/main/pad-tools/pad-init-tools.tgz"

echo "Download the files and unzip it"
wget -qO- ${DOWNLOAD_URI} | tar zx

echo "Jump to the deploy.sh"
bash pad-init-tools/deploy.sh

echo "End of call and clear cache"
rm -rf pad-init-tools
