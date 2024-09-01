This is a template that utilizes Docker, Rust actix Web, and any front end language of your choice. 

Featuers: 
Highly performant, typesafe, backend web framework. 
Docker image is 8 mb fully compiled and can be hosted anywhere that supports containers. 
Tracing and logging built in.

Full description: 
This docker container can be used to imbed into a rust Actix Web Binary the static files generated from any of your favorite web frameworks, and also giving you a full back end web service with tracing. 

What makes this unique is it is fast, and immutable. Instead of storing the files that you would serve to clients in a direcotry on the server, the files are actually built directly into the binary, reducing your attack surface for directory traversal and offering a huge performance increase as you do not have to my calls down to the underlying OS to read files to send or store them in a cache, they are built directly into the executable. 

To test this project make sure that you have docker installed, navigate to the root of the project and enter 
```
docker compose up --build
```

There is also a configuration file specifically for setting up a data base on the same host, or a different one. 
