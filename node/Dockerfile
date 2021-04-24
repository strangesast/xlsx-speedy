arg tag=latest
from node:${tag}

workdir /usr/src/app
copy package*.json ./
run npm ci
copy main.js .
cmd ["node", "main.js"]
