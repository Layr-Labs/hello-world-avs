FROM node:14

WORKDIR /app

COPY package.json ./
COPY tsconfig.json ./
RUN npm install

COPY . .

CMD ["npm", "start"]