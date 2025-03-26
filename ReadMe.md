<!-- PROJECT SHIELDS -->
<!--
* I'm using markdown "reference style" links for readability.
* Reference links are enclosed in brackets [ ] instead of parentheses ( ).
* See the bottom of this document for the declaration of the reference variables
* for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
* https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

![](https://img.shields.io/badge/Personal_Project-blue)


# Rust Api Server

> Are you a rustcean and have you created you own crates? Well we keep track of various rustaceans and the crates they have! How cool?

## Technologies

> Rust programming language.

Below are some of the key libraries and tools used used/applied in this project include:

- Rust programming languages - I'll say it again
- Docker - Containerization
- Redis - Caching
- PogreSql - Database
- Diesel - Database and Model Management
- Serde - Serialization and Deserialization
- Rocket - Request handling
- Reqwest (For testing)

### Main Project Features
The project is divided into two main application binaries, there server and the CLI.

#### Server Binary

- Authentication
- Authorization
- Route handling
- GET, POST, PUT, DELETE & OPTIONS methods on `/crates`, `/rustaceans` and `/login` endpoints

#### CLI Binary

- User management
- Emailing

## Getting Started

### Prerequisites
- A linux based Operating System
- Docker
- Docker Compose

To get a local copy up and running follow these simple example steps.

```bash
git clone https://github.com/RyanKoech/rust_api_server.git
```
Build and start services with Docker Compose

```bash
docker compose up --build -d
```

Build the rust binaries
```bash
docker compose exec app cargo build  
```

Create a user via the cli
```bash
docker-compose exec app cargo run --bin cli users create admin 1234 admin
```

Run the server
```bash
docker compose exec app cargo run  

```

In a separate terminal login to the api
```bash
docker compose exec app curl http://127.0.0.1:8000/login -H 'Content-type: application/json' -H 'Accept: application/json'  -d '{"username": "admin", "password": "1234"}'

```

Explore the rest of the methods and endpoints; GET, POST, PUT, DELETE & OPTIONS methods on `/crates` and `/rustaceans`

### Testing

To run tests
```bash
docker compose exec app cargo test

```

## Show your support

Give a ⭐ if you like this project!

## 📝 License

This project is [MIT](./LICENCE) licensed.


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/RyanKoech/rust_api_server.svg?style=for-the-badge
[contributors-url]: https://github.com/RyanKoech/rust_api_server/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/RyanKoech/rust_api_server.svg?style=for-the-badge
[forks-url]: https://github.com/RyanKoech/rust_api_server/network/members
[stars-shield]: https://img.shields.io/github/stars/RyanKoech/rust_api_server.svg?style=for-the-badge
[stars-url]: https://github.com/RyanKoech/rust_api_server/stargazers
[issues-shield]: https://img.shields.io/github/issues/RyanKoech/rust_api_server.svg?style=for-the-badge
[issues-url]: https://github.com/RyanKoech/rust_api_server/issues
[license-shield]: https://img.shields.io/github/license/RyanKoech/rust_api_server.svg?style=for-the-badge
[license-url]: https://github.com/RyanKoech/rust_api_server/blob/master/LICENCE
