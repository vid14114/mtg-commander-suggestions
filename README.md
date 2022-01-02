# mtg-commander-suggestions
mtg-commander-suggestions reads a csv export of a deckbox.org collection and tries to find sets of cards that possibly play well with abilities of potential Commanders in the collection

## Prerequisites
* MongoDB (see docker-compose.yml)

## Operating principle
1. Update local copy of Scryfall Oracle Cards database
2. Read collection csv file from stdin and match to oracle cards
3. Filter possible Commanders and extract keywords
4. Group cards by keywords 

## Todo
5. Count other common themes among grouped cards