FROM node:23-alpine
WORKDIR usr/src/frontend
COPY . .
# install dependencies
RUN npm i
RUN npm run build
EXPOSE 8080

# build frontend bundle
CMD npx http-server dist/ -o -a --cors -p 8080
