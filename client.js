const { WebSocketServer } = require("./index.js"); 

const port = 3001;
const server = new WebSocketServer(port, (id, err) => {
    console.log(id, err);
    //console.log(`Server listem on ${port}`);
});