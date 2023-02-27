const fs = require("fs");

fs.writeFileSync("./src/encryption", process.env.KEY);