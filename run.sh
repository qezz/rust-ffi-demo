#!/bin/bash

case "$OSTYPE" in
  darwin*)  DYLD_LIBRARY_PATH=./c cargo run ;; 
  linux*)   LD_LIBRARY_PATH=./c cargo run ;;
  *)        echo "Your platform is not supported. Please use Linux or macOS" ;;
esac

