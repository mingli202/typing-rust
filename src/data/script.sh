#!/bin/bash

homedir="$HOME/dev/Codes/Rust/typing_test"

# get words from monkeytype
cp "$homedir/src/data/words.json" "$homedir/src/data/words.json.bak.bak"
cp "$homedir/src/data/words.json.bak" tmp.json

for name in "" "_1k" "_5k" "_10k" "_25k" "_450k" "_commonly_misspelled" "_doubleletter" "_medical" "_shakespearean"; do
	curl "https://raw.githubusercontent.com/monkeytypegame/monkeytype/refs/heads/master/frontend/static/languages/english$name.json" | jq '.words' | jq -s -c '.[0] + .[1]' tmp.json - >tmps.json
	mv tmps.json tmp.json
done

curl 'https://raw.githubusercontent.com/monkeytypegame/monkeytype/refs/heads/master/frontend/static/quotes/english.json' | jq '.quotes | map(.text|split(" ")) | flatten' | jq -s -c '.[0] + .[1]' tmp.json - >tmps.json
mv tmps.json tmp.json

jq -c "map(ascii_downcase) | unique" tmp.json >tmps.json
mv tmps.json tmp.json

mv tmp.json "$homedir/src/data/words.json"
jq 'length' "$homedir/src/data/words.json"

# get quotes from monkeytype
# cp "$homedir/src/data/quotes.json" "$homedir/src/data/quotes.json.bak.bak"
# cp "$homedir/src/data/quotes.json.bak" tmp.json
#
# curl 'https://raw.githubusercontent.com/monkeytypegame/monkeytype/refs/heads/master/frontend/static/quotes/english.json' | jq '.quotes | map(.text|split(" ")) | flatten' | jq -s -c '.[0] + .[1]' tmp.json - >tmps.json
