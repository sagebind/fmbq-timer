FROM sandrokeil/typescript
COPY . /app
RUN make

FROM nginx
COPY --from=0 /app/dist /usr/share/nginx/html
