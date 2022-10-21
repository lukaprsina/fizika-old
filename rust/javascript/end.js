const json = JSON.stringify(Serialize, null, 2);
fs.writeFileSync('output.json', json, 'utf-8')