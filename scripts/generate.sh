#!/bin/sh
openapi-generator generate -i https://web.homechart.app/swagger.yaml -g rust -c generateConfig.json