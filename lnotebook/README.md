<h1 align ="center">LNotebook</h1>
<div align ="center">

asynchronous API that will helps you to write simple notebook

<a href="https://crates.io/crates/lnotebook">
<img src="https://img.shields.io/crates/v/lnotebook"/>
</a>
<a href="https://docs.rs/lnotebook/">
<img src="https://img.shields.io/docsrs/lnotebook">
</a>
</div>

## Important
<strong>
I don't recommend using this API in large projects because it is a too
simple and inflexible API (one of the reasons is this is my first API).
</strong>

<h3>This API using:</h3>

- **[SQLx](https://github.com/launchbadge/sqlx?tab=readme-ov-file)**
- **[Tokio](https://tokio.rs/)**

## Preparing
Before start working with notebook you must complete commands below.

1. Clone our repository:
```
git clone https://github.com/Nnenty/lnotebook
```
2. Export your database (change URL on yours):
```
export DATABASE_URL=postgres://username:password@localhost/db`
```
3. Migrate using command:
```
cd lnotebook/
sqlx migrate run
```
> **Note**: install [SQLx-cli](https://crates.io/crates/sqlx-cli) if you don't have it installed to run code above.

<h4> Then you have a database ready to use in notebook. </h4s>

## Start the notebook
> **Clarification**: all code in this section assumes that you have completed all commands from the section [Preparing](https://github.com/Nnenty/lnotebook_api?tab=readme-ov-file#preparing).
- First, go to [notebook_example](https://github.com/Nnenty/lnotebook/tree/master/notebook_example) directory:
```
cd notebook_example/
```

- Let's try execute `cargo run` only:
```
cargo run
```
> **Note**: when you use `cargo run` without terminal command, program should display all total notes.
Read more about terminal commands in [our documentation](https://docs.rs/lnotebook/latest/lnotebook/commands/execute_commands/).

Let's add new note:
```
cargo run -- add-note passwords
```
The program will ask you to enter the desired note to add to the notebook. Paste text
`login: krutoy_4el
password: 1234#endnote#`
into.

Then let's print our note:
```
cargo run -- display-note passwords
```
The output of this program will be like this:
```
ID: 1
Name: passwords
Data:
login: krutoy_4el
password: 1234
```

### More about terminal commands
To learn more about commands in terminal similar to `add-note` from the example above read [our documentation](https://docs.rs/lnotebook/latest/lnotebook/commands/execute_commands/).

## Licenses
Licensed under either license:
- Apache-2.0 License
- MIT License