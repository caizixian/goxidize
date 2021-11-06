# Goxidize
A link shortener.
## Perquisites
- Rust 1.56+
- npm CLI and Node.js
- Any officially supported PostgreSQL

## Build
Before building the project, please create a `.env` file with a valid PostgreSQL URL, such as the following.
```text
DATABASE_URL=postgresql://localhost:5432/goxidize
```
Then, run the database migrations.
```bash
sqlx migrate run
```
This database is queried against for compile-time checks done by `sqlx`.

If you don't have a database set up, `sqlx` will check against `sqlx-data.json`.
To generate this file, run `cargo sqlx prepare -- --lib` and `cargo sqlx prepare --check -- --lib`.
Note that you will still need a working database to run the tests.

### Development
The Parcel development server supports auto-reloading for frontend assets.
Use the following file to let the Parcel server proxy requests to the backend.
```javascript
const { createProxyMiddleware } = require("http-proxy-middleware");

module.exports = function (app) {
  app.use(
      createProxyMiddleware(["/**", "!/", "!**/*.html", "!**/*.js", "!*.css", "!**/*.css", "!**/*.map"], {
        target: "http://localhost:8000/",
      })
  );
};
```

Run the following to start the servers.
```bash
cargo run # use cargo watch for auto-reloading
# then on a separate terminal, run
# `npm install` if you have not installed the dependencies, and
npm start
```

### Production
```bash
npm run build
```

## Configuration
A YAML file is used for configuration.
This affects the run-time behaviour of the program.
```yaml
# configuration.yml
port: 8000
database:
  url: postgresql://localhost:5432
  name: goxidize
debug: true
```

## Deployment
An example of deployment using `docker-compose` is provided below.
```yaml
# docker-compose.yml
version: '3'
services:
  db:
    image: postgres:14
    restart: always
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
      POSTGRES_DB: goxidize
      LANG: C.UTF-8
    volumes:
      - dbdata:/var/lib/postgresql/data
  goxidize:
    image: ghcr.io/caizixian/goxidize:master
    restart: always
    depends_on:
      - "db"
    ports:
      - "8000:8000"
    # Using environment variables is another option 
    volumes:
      - "./goxidize_config.yml:/goxidize/configuration.yml"

volumes:
  dbdata:
```

```yaml
# goxidize_config.yml
host: "0.0.0.0"
port: 8000
database:
  host: db
  port: 5432
  name: goxidize
debug: false
```

## License
Except as otherwise noted (e.g., in individual files), the project is
licensed under the Apache License, Version 2.0 [LICENSE-APACHE](./LICENSE-APACHE) or
<http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
[LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>, at your option.
