# Rust Login

A login/register system written in Rust + ReactJS.

## Features
### Backend
Technologies: Rust, Iron, Diesel
- User registration (passwords are hashed + salted)
- User login
- User logout
- User updating
- User deleting
- User retrieving

### Frontend
Technologies: ReactJS, Brunch, Glamorous
- Homepage with login/register form
- Settings page with user edit form & user delete button

## Requirements
- PostgreSQL installed
- NodeJS v7.8.x or higher installed
- Diesel CLI v0.15.x or higher installed
- Rust v1.19.x or higher installed

## Setup
- Clone the repository
- `cd rust_login`
- Set the DATABASE_URL in .env
  `echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
`
- Run `diesel setup`
- Run `npm install`
- Run `brunch build -p` to compile the JavaScript
- Run `cargo run`
