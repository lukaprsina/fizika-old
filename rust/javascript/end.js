const json = JSON.stringify(output, null, 2);
fs.writeFileSync('output.json', json, 'utf-8');

if (BOOK_COUNT !== 0)
    console.log("Book count:", BOOK_COUNT)

if (UNSCRAMBLED_MULTICHOICE_COUNT !== 0)
    console.log("Unscrambled count:", UNSCRAMBLED_MULTICHOICE_COUNT)
