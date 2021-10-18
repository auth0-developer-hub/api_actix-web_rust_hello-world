# Hello World API: Actix Web + Rust Sample

You can use this sample project to learn how to secure a simple [Actix Web][actix] API server using Auth0.

## Get Started

Install the project dependencies:

```bash
cargo build
```

The following settings are configurable via environment variables:

- `CLIENT_ORIGIN_URL` - CORS allowed origin (default `http://localhost:4040`)
- `PORT` - Application port (default `6060`)
- `RUST_LOG` - Log level (default `info`)

```bash
cp .env.example .env
```

Edit the content of `.env` accordingly.

**Note:** The previous step is optional since all settings have a default value.

Run the project in dev mode:

```bash
cargo run
```

## API Endpoints

The API server defines the following endpoints:

### 🔓 Get public message

```bash
GET /api/messages/public
```

#### Response

```bash
Status: 200 OK
```

```json
{
  "message": "The API doesn't require an access token to share this message."
}
```

### 🔓 Get protected message

> You need to protect this endpoint using Auth0.

```bash
GET /api/messages/protected
```

#### Response

```bash
Status: 200 OK
```

```json
{
  "message": "The API successfully validated your access token."
}
```

### 🔓 Get admin message

> You need to protect this endpoint using Auth0 and Role-Based Access Control (RBAC).

```bash
GET /api/messages/admin
```

#### Response

```bash
Status: 200 OK
```

```json
{
  "message": "The API successfully recognized you as an admin."
}
```

## Error Handling

### 400s errors

#### Response

```bash
Status: Corresponding 400 status code
```

```json
{
  "message": "Message that describes the error that took place."
}
```

### 500s errors

#### Response

```bash
Status: 500 Internal Server Error
```

```json
{
  "message": "Message that describes the error that took place."
}
```

[actix]: https://actix.rs/
