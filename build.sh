#!/bin/bash
docker rmi yunchao_plus_wallet
docker rm yunchao_plus_wallet
npm run build
docker build -t yunchao_plus_wallet .
