const http = require('http');
const fs = require('fs');

const server = http.createServer(function(req, res) {
    process.req = req;
    let body = "";
    req.setEncoding('utf8');
    req.on('data', function(chunk){
        body+=(chunk);
    });
    req.on('end', function() {
        console.log(body);
        fs.writeFileSync(Date.now() + ".log", JSON.stringify(body));

    })

    res.end("ok");
});

server.listen(8080);