FROM node:26-alpine AS app-builder
WORKDIR /app

ARG APP_API_URL
ARG APP_WS_URL

ENV APP_API_URL=$APP_API_URL
ENV APP_WS_URL=$APP_WS_URL

COPY ./app/package.json ./app/package-lock.json* ./
RUN npm install

COPY ./app ./
RUN npm run build

FROM nginx:alpine

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=app-builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
