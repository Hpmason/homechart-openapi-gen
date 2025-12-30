#!/bin/sh
openapi-generator generate -i https://web.homechart.app/swagger.yaml -g rust --api-package homechart-api --model-package homechart-models --package-name homechart-api