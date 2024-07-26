# rust-postgres
 

## SETUP


### PostgreSQL

A quick [video](https://www.youtube.com/watch?v=2P5guvvqP5Y) on installing PostgreSQL and pgAdmin on Mac.
The process should be similar for other platforms.   

Go to [EDB](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads) to grab the proper package for your needs.

**Important**: take a note of the password and port number when you install your database.  
It is common to set the main user as `postgres` and the password for it as `postgres` as well.  
 The conventional port number is `5432`.


### Rust

If somehow you ended up here and don't have Rust locally installed - [get it now](https://www.rust-lang.org/tools/install).

#### Environment

Open the `.env` file and edit it as needed. If you named your user `postgres`, set the password to `postgres` and the port number to `5432` - awesome. You don't have to change anything.


### Postman

If you don't have a [Postman](https://www.postman.com/) account - get it. I have included the `rest-postgres.postman_collection.json` file that contains my Postman collection. Use it if you want.  

Alternatively, you may use `curl` or whatever you like to get data to and from an http endpoint.



## RUNNING APP

If you want to see code in action, you'll need to start a PostgreSQL server, an http server and send requests from Postman.

### PostgreSQL <--> Rust

1. Start PostgreSQL server first. I use pgAdmin for that, but you can use whatever you like.

2. Now that the server is running, let's start the http server. In your terminal type `cargo run`.

3. Make sure the http server can "talk" to your database. Look at the terminal. If everything is connected properly, you should see this message: `Connection to the database established!` If you see any errors, check these two things: 
  - is the PostgreSQL server running?
  - is the username, password and port number in the `.env` file correct?

### Rust <--> Postman

1. Go to your web browser and type `http://localhost:8080/messages/list` - you you get an empty messages collection - great. If you receive any kind of error instead, like "page not found" or something else - make sure your http server is running and not printing any errors in the terminal.

2. Now go to Postman. If you have imported my collection `rest-postgres.postman_collection.json` - you probably know what to do. If not - you'll have to create two requests.  

First create a GET request to `http://localhost:8080/messages/list`. Send it to the server and make sure you receive the empty messages collection, just like in the web browser. If this is the case - cool, move on. 

If instead you receive an error or, worse, nothing - check [this post](https://community.postman.com/t/post-to-localhost/13236/9). I had this problem myself and had to instal a Postman agent for the localhost.

### Create and Query Messages in Postman

By now you have probably created a GET request to `http://localhost:8080/messages/list`. Time to create our second request - a POST request to `http://localhost:8080/messages/create`. You will have to send some data in your request. So, go to "Body" tab, choose "raw" radio button and in the textfield type something like this:
```
{
    "content": "test"
}
```
Of course, you can type other words and sentences, it doesn't have to be "test".

Alternate GET and POST requests and you should see how your database is populated with all your wonderful test data :D