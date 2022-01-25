var http = require("http");

var server = http.createServer(function (req, res) {

     if (req.url == '/') { //check the URL of the current request
        
        // set response header
        res.writeHead(200, { 'Content-Type': 'text/html' }); 
        
        // set response content    
        res.write('<html><body><p>This is a home Page please try /payment.</p></body></html>');
        res.end();
    
    } 
    if (req.url == '/payment') { //check the URL of the current request
        
        // set response header
        //res.writeHead(200, { 'Content-Type': 'text/html' }); 
        
        // set response content

	var y = Math.random();
	if (y < 0.5)
  		y = 0
	else
  		y = 1
   
        res.send(y);
        res.end();
    
    }
    else
        res.end('Invalid Request!');



}).listen(8081);

console.log('Server running at http://127.0.0.1:8081/');


// Console will print the message

