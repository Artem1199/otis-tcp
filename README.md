# otis-tcp

Receives data from arduino using otis-arduino code through TCP, and stores in SQL database.

1. Install [Diesel_cli](http://diesel.rs/guides/getting-started/) for mysql
   1. For mysql use: `cargo install diesel_cli --no-default-features --features mysql`
   2. You may also need `sudo apt-get install libmysqlclient-dev` and `sudo apt-get install libmysqlclient21`
   3. Install mysql and setup a username/password.  Create a database, for example: `otis_data`
   4. Setup diesel `diesel setup --database-url mysql://username:password@127.0.0.1:3306/otis_data`
   5. Run `diesel migration generate create_posts` to create posts that are used to create and delete tables from database
   6. Replace the migrations with the ones from this repo
   7. Run `diesel migration run`
2. Update the addresses for your system in `main.rs`
2. `cargo run` to execute the program.
3. If you're arduino is properly setup to send data then this should communicate with `otis-arduino` software

Requires diesel_cli, mysql

1. mysql database has to be setup for diesel migration
2. .env file used to store mysql connection with diesel is not shown
2. Arduino IP address hardcoded in main if change req.
