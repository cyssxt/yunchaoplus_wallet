FROM node:14.15.5-alpine
WORKDIR /app
COPY package.json ./
COPY package-lock.json ./
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories
RUN apk add --update python make g++\
   && rm -rf /var/cache/apk/*
RUN npm config set registry https://registry.npm.taobao.org  && npm install
COPY dist ./
EXPOSE 8080
CMD 'node' 'app.js'

