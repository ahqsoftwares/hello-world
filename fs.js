const fs = require("fs");

fs.writeSync("./src/encryption", process.env.KEY);