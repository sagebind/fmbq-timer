FROM sandrokeil/typescript
COPY . /app
RUN make

FROM nginx
COPY --from=0 /app /usr/share/nginx/html
