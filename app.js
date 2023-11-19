// Serve the Dist folder

let express = require('express');

let path = require('path');

app = express();

app.configure(function(){
    app.use(express.bodyParser());
    app.use(express.static(path.join(__dirname, 'dist')));
});

let port = process.env.PORT || 5000;

app.listen(port);

console.log('server started ' + port);