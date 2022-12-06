const fs = require('fs');

fs.readFile('../input/day1.txt', 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }

    let elves = []

    let elf_calories = 0;

    const lines = data.split("\n");
    for (let i = 0; i < lines.length; i ++) {
        if (lines[i] === '') {
            elves.push(elf_calories);
            elf_calories = 0;
        } else {
            elf_calories += parseInt(lines[i]);
        }
    }

    elves.sort((a, b) => b-a);

    console.log(`Part 1: ${elves[0]}`);
    console.log(`Part 2: ${elves[0] + elves[1] + elves[2]}`)
});
