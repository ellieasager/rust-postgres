# Demo of Rust + PostgreSQL + Uuid
 
This is companion code to [my article on dev.to](https://dev.to/ellie_sager_elliecat/rusty-recipies-sqlx-uuid-477m). It shows how you can use `sqlx` library when you choose to use `Uuid` as your `id` type rather than an integer.


## SETUP


### Docker

Please install [Docker](https://docs.docker.com/engine/install/).


### Postman

If you don't have a [Postman](https://www.postman.com/) account - get it. I have included the `rest-postgres.postman_collection.json` file that contains my Postman collection. Use it if you want.  

Alternatively, you may use `curl` or whatever you like to get data to and from an http endpoint.



## RUNNING APP

If you want to see code in action, you'll need to start a PostgreSQL server, an http server and send requests from Postman.

### PostgreSQL <--> Rust

After I dockerized my initial project, all you have to do now is type: `docker compose up --build`.  
Let me know if you have any issues.

### Rust <--> Postman

1. Go to your web browser and type `http://localhost:8080/` - you you get "Hello World!" message - great. If you receive any kind of error instead, like "page not found" or something else - make sure your docker container is running and not printing any errors in the terminal.

2. Now go to Postman. If you have imported my collection `rest-postgres.postman_collection.json` - you probably know what to do. No need to read the rest of this README.  
If not - you'll have to create two requests.  

### Create and Query Messages in Postman

First create a GET request to `http://localhost:8080/messages`. Send it to the server and make sure you receive the empty messages collection, just like in the web browser. If this is the case - cool, move on. 

If instead you receive an error or, worse, nothing - check [this post](https://community.postman.com/t/post-to-localhost/13236/9). I had this problem myself and had to instal a Postman agent for the localhost.

Time to create our second request - a POST request to `http://localhost:8080/messages/create`. You will have to send some data in your request. So, go to "Body" tab, choose "raw" radio button and in the textfield type something like this:
```
{
    "content": "test"
}
```
Of course, you can type other words and sentences, it doesn't have to be "test".

Alternate GET and POST requests and you should see how your database is populated with all your wonderful test data :D
