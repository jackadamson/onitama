# Onitama App

Can be played at [https://onitama.mrfluffy.dev/](https://onitama.mrfluffy.dev/)

## Roadmap

Things that could be cool to implement, that aren't done yet, and might one day get done

- [ ] Say when opponent has requested a rematch
- [ ] Add chat (maybe)
- [ ] Add "how to play"
- [ ] Add different difficulty AIs
- [ ] Show piece that last moved, and where it moved from

## Deployment

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
