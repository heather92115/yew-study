#!/usr/bin/env sh

curl -i \
-H "Accept: application/json" \
-H "Content-Type: application/json" \
-X POST -d '{"variables":{"awesomeId":1,"limit":3},"query":"query VocabList($awesomeId: Int!, $limit: Int!) {\n  getStudyList(awesomeId: $awesomeId, limit: $limit) {\n    vocabId\n    vocabStudyId\n    prompt\n  }\n}\n","operationName":"VocabList"}' \
http://127.0.0.1:3001/gql