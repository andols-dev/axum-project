# Rust Axum Project

A full-stack web application built with **Rust**, **Axum**, and **React**. The project is currently under active development.

## Tech Stack

### Backend

- Rust
- Axum
- SQLx
- SQLite
- Tokio
- Tower HTTP
- Tower Cookies

### Frontend

- React
- Vite
- JavaScript
- CSS

## Features

### ✅ Implemented

- User registration
- Secure password hashing with Argon2
- User login
- Session-based authentication
- HTTP-only cookies
- Logout functionality
- SQLite database
- SQLx database migrations

## Getting Started

### Clone the repository

```bash
git clone <repository-url>
cd <project-folder>
```

### Backend

Install the Rust toolchain and SQLx CLI.

Run the database migrations:

```bash
sqlx migrate run
```

Start the backend:

```bash
cargo run
```

The backend runs on:

```text
http://localhost:3000
```

### Frontend

Install dependencies:

```bash
npm install
```

Start the development server:

```bash
npm run dev
```

The frontend runs on:

```text
http://localhost:5173
```

## Authentication

Authentication is implemented using server-side sessions.

Login flow:

1. The password is verified using Argon2.
2. A secure random session token is generated.
3. The session is stored in the SQLite database.
4. The token is sent to the browser as an HTTP-only cookie.
5. Protected routes validate the session against the database.

## Status

🚧 This project is currently under active development.

The project is continuously evolving as new features and improvements are implemented.
