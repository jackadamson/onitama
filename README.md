# Onitama App
[![Docker Image CI](https://github.com/jackadamson/onitama/actions/workflows/docker-image.yml/badge.svg)](https://github.com/jackadamson/onitama/pkgs/container/onitama)

Can be played at [https://onitama.app/](https://onitama.app/)

## Roadmap

Things that could be cool to implement, that aren't done yet, and might one day get done

- [x] Show piece that last moved, and where it moved from
- [x] Say when opponent has requested a rematch
- [ ] Add chat (maybe)
- [x] Add "how to play"
- [x] Add different difficulty AIs

## Local AI

The default Dockerfile will disable server-side AI, and instead compile the AI Agent as web-assembly and run in a
JS WebWorker.

An alternate Dockerfile (`Dockerfile.remoteai`) uses server-side AI which runs the agent code in the same process that
serves the game and delivers messages for multiplayer games.

~~The server-side AI is roughly 30x faster at running Monte Carlo simulations, meaning that the hard bot is notably harder when run server side.~~  
The server side AI is roughly 2x faster, ever since swapping out the RNG used in the Monte Carlo search. Previously it was
significantly slower.

To use server-side AI, build the container with
```bash
docker build -t onitama:remoteai -f Dockerfile.remoteai .
```

As of writing this, [https://onitama.app/](https://onitama.app/) uses the **local AI** as it is
very light on server resource requirements.

## Deployment

### Run from GitHub

Pull from GitHub container registry
```bash
docker pull ghcr.io/jackadamson/onitama:latest
```

Run the container
```bash
docker run -dp 80:8080 --name onitama --rm ghcr.io/jackadamson/onitama:latest
```

### Build Locally

Build the container
```bash
docker build -t onitama .
```

Run the container
```bash
docker run -dp 80:8080 --name onitama --rm onitama
```

## Development

Requires Rust (nightly) and Node (v14)  

1. Install dependencies with `yarn install`  
1. Start the backend with `cargo run onitamaserver`
1. Start the frontend with `yarn start`
1. Visit [http://localhost:3000](http://localhost:3000) to see the app

To develop single-player without the backend, start the frontend with
```bash
yarn start-local-ai
```
