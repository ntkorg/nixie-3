const data = await Bun.file("dump.txt").text();
console.log(data.split("\n").map((line) => line.replaceAll(" ", "").substring(12)).filter((line) => line.length > 0).join(""));
