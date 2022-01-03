# mtg-commander-suggestions
mtg-commander-suggestions reads a csv export of a deckbox.org collection (actually any csv with a 'Name' column) and tries to find sets of cards that possibly play well with abilities of potential Commanders in the collection

## Prerequisites
* MongoDB (see `docker-compose.yml`)

## Operating principle
1. Update local copy of Scryfall Oracle Cards database
2. Read collection csv file and match to oracle cards
3. Filter possible Commanders and extract keywords
4. Group cards by keywords 

## Run
```
cargo run -- collection.csv
```
or docker on Linux
```
docker run --rm -v ${PWD}:/usr/src/myapp -w /usr/src/myapp rust:1.57.0 cargo build --release
./target/release/mtg-commander-suggestions collection.csv
```
or docker on Windows
```
docker build -t rust-x86_64-pc-windows-gnu .
docker run --rm -v ${PWD}:/usr/src/myapp -w /usr/src/myapp rust-x86_64-pc-windows-gnu cargo build --target x86_64-pc-windows-gnu --release
.\target\x86_64-pc-windows-gnu\release\mtg-commander-suggestions.exe collection.csv
```

## Todo
* Count other common themes among grouped cards