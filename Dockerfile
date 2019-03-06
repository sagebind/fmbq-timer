FROM sandrokeil/typescript
COPY . /app
RUN make && mv dist /dist

FROM nginx
COPY --from=0 /dist /usr/share/nginx/html
