#!/bin/sh
cat .openapi-generator/FILES | xargs rm
find . -type d -empty -delete