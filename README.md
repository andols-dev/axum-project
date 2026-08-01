# Personal Knowledge Management System

This project is a web application currently under development. Its goal is to provide a digital knowledge base where users can organize, store, and retrieve information in one central place.

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
- Tailwind CSS

## Features

### ✅ Implemented features

- User registration
- Secure password hashing with Argon2
- User login
- Session-based authentication
- HTTP-only cookies
- Logout functionality
- SQLite database
- SQLx database migrations

### Future features

- Create, read, update, and delete notes
- Organize notes with tags and categories
- Mark notes as favorites
- Search saved notes
- Markdown support for note formatting

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
