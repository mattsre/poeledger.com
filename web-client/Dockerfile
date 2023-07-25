FROM node:lts AS deps
WORKDIR /app

ENV PORT="3000"
ENV NODE_ENV="production"
EXPOSE 3000

COPY package.json package-lock.json .
RUN npm install --include=dev

COPY . .
RUN npm run build

CMD ["npm", "run", "start"]
