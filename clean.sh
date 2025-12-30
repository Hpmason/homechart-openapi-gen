#!/bin/sh
cd homechart-openapi-gen && cat .openapi-generator/FILES | xargs rm
find . -type d -empty -delete