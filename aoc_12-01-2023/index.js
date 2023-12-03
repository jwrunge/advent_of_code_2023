const fs = require("fs");
const words = { "one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8, "nine": 9 };


const lines = fs.readFileSync("./coords.txt", "utf8").split("\r\n");
let total = 0;

function getFirstOrLastNumber(line, last) {
    const newLine = last ? line.split("").reverse().join("") : structuredClone(line);
    let i = 0;
    for(let char of newLine) {
        const parsed = parseInt(char); 
        if(isNaN(parsed)) {
            for(const word in words) {
                const substr = line.slice(last ? line.length - i - word.length : i, last ? line.length - i : i + word.length);
                if(substr === word) {
                    return words[word];
                }
            }
        } else {
            return parsed;
        }

        i++;
    }
}

for(let line of lines) {
    let newNum = `${getFirstOrLastNumber(line)}${getFirstOrLastNumber(line, true)}`
    total += parseInt(newNum) || 0;
}

console.log(total);
