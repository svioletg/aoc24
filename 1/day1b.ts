export { };

const sample: string = `3   4
4   3
2   5
1   3
3   9
3   3`;

const puzzle: string = await Deno.readTextFile("input.txt");

var left: string[] = [];
var right: string[] = [];

for (const line of puzzle.split("\n")) {
    var pair: string[] = [];

    for (const m of line.matchAll(/(\S+)/g)) {
        pair.push(m[0]);
    }

    if (pair.length < 2) {
        break;
    }

    left.push(pair[0]);
    right.push(pair[1]);
}

left.sort();
right.sort();

var total: number = 0;
var simscore: number = 0;

for (var i = 0; i < left.length; i++) {
    var l: number = Number(left[i]);
    var r: number = Number(right[i]);
    total += Math.abs(l - r);
    simscore += l * right.filter((i) => { return Number(i) === l ? true : false; }).length;
}

console.log(simscore);
