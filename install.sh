#!/bin/sh

set -eu

install () {

  UNAME=$(uname)
  ARCH=$(uname -m)
  if [ "$UNAME" != "Linux" -a "$UNAME" != "Darwin" ] ; then
    echo "Error."
    exit 1
  fi
  
  if [ "${ARCH}" != "x86_64" ] ; then
    echo "Error."
    exit 1
  fi

  if [ "$UNAME" = "Darwin" ] ; then
    PLATFORM="x86_64-apple-darwin"
  elif [ "$UNAME" = "Linux" ] ; then
    PLATFORM="x86_64-unknown-linux-gnu"
  fi

  LATEST=$(curl -s https://api.github.com/repos/micin-jp/racco/tags | grep name  | head -n 1 | sed 's/[," ]//g' | cut -d ':' -f 2)
  URL="https://github.com/micin-jp/racco/releases/download/${LATEST}/racco-${LATEST}-${PLATFORM}.zip"
  DEST=${DEST:-/usr/local/bin/racco}

  if [ -z $LATEST ] ; then
    echo "Error."
    exit 1
  fi

  TEMPDIR="/tmp"
  TEMP="${TEMPDIR}/racco-${LATEST}-${PLATFORM}"

  echo "Downloading: ${URL}"

  curl -sL "$URL" -o "${TEMP}.zip"
  unzip -o "${TEMP}.zip" -d "$TEMPDIR"
  cp "${TEMP}/racco" "$DEST"

  echo "Installation was sucessful."
}

install

