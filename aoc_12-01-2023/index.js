const fs = require("fs");
const words = { "one": "1", "two": "2", "three": "3", "four": "4", "five": "5", "six": "6", "seven": "7", "eight": "8", "nine": "9" };
const lines = fs.readFileSync("./coords.txt", "utf8").split("\r\n");
let total = 0;

function getFirstOrLastNumber(line, last) {
    console.log(line, last);
    if(last) line = line.split().reverse().join();
    console.log(line);
    for(let char of line) {
        try { 
            console.log(char, parseInt(char))
            return parseInt(char); 
        }
        catch(_) { true }
    }
}

for(let line of lines) {
    let newLine = "";
    for(let i=0; i < line.length; i++) {
        for(const word in words) {
            const substr = line.slice(i, i + word.length);
            if(substr === word) {
                newLine += words[word];
            }
        }
        newLine += line[i];
    }
    let newNum = `${getFirstOrLastNumber(newLine)}${getFirstOrLastNumber(newLine, true)}`
    total += parseInt(newNum) || 0;
}

console.log(total);
