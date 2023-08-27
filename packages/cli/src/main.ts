import "dotenv/config";
import fs from "fs";

let content = fs.readFileSync("src/input.rust", "utf8");

// convert `columns` set to array
content = content.replaceAll(/(columns: )\{(\n(?: {16}.*\n)+)( {12}})/g, '$1[$2            ]');

content = content.replaceAll(/({\s*)([^:]*)\s*\(/g, '$1"$2": (');

content = content.replaceAll(/([\w\d]+) ?{/g, "{ type: $1,");
content = content.replaceAll("): [", "");

content = content.replaceAll(/([\w\d]+)\(/g, '[ "$1",');
content = content.replaceAll(/\(/g, "[");
content = content.replaceAll(/\)/g, "]");

content = content.replace(/(['"])?([\w\d]+)(['"])?:\s*/g, '"$2": ');
content = content.replace(/:\s*([\w\d\.]+)/g, ': "$1"');
content = content.replace(/,(\s*[\}\]])/g, '$1');
content = content.replace(/(0x[\da-f]+)/g, '"$1"');
content = content.replace(/^(\s*)([\w\d]+)/gm, '$1"$2"');

// console.log(content);
fs.writeFileSync("../ui/src/assets/output.json", content);
