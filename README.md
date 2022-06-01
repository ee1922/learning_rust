# Learning-rust
Building a restaurant API using Rust


## Build and Run

You need to have Rust and Cargo installed.

Run the server:

```
$ cd rust-restaurant-api
$ cargo run
```

Run the client:

```
$ cd rust-restaurant-api
$ cargo run
```


## API Design

- `POST /item`: add an item on the certain table
- `DELETE /item?table_id=:table_id&&item_id=:item_id` delete the certain item on the certain table
- `GET /item?table_id=:table_id&&item_id=:item_id`: check if the certain item on the certain table
- `GET /item?table_id=:table_id`: show all items on the certain table
