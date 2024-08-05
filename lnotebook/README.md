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

---

<strong>
I don’t recommend using this API, because it could be done much better
</strong>

<h3>This API using:</h3>

- **[SQLx](https://github.com/launchbadge/sqlx?tab=readme-ov-file)**
- **[Tokio](https://tokio.rs/)**

## Preparing
Before start working with notebook you must complete commands below.

1. Clone our repository:
```
git clone https://github.com/Nnenty/lnotebook

cd lnotebook/
```
2. Go to [notebook_example](https://github.com/Nnenty/lnotebook/tree/master/notebook_example) directory:
```
cd notebook_example/
```
3. Export your database (change URL on yours):
```
export DATABASE_URL=postgres://username:password@localhost/db`
```
4. Migrate using command:
```
just migrate
```
> **Note**: install [SQLx-cli](https://crates.io/crates/sqlx-cli) if you don't have it installed to run code above.

<h4> You did it! Then you have a database ready to use in notebook. </h4s>

## Start the notebook
> **Clarification**: all code in this section assumes that you have completed all commands from the section [Preparing](https://github.com/Nnenty/lnotebook_api?tab=readme-ov-file#preparing).

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