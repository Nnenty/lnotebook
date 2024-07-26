<h1 align ="center">LNotebook</h1>
<div align ="center">
<h3> asynchronous API to creating notebooks that stores notes in a database </h3>

<h5> API that will helps you quickly make your own notebook and commands to manipulate it </h5>
</div>

<h3>This API using:</h3>

- **[SQLx](https://github.com/launchbadge/sqlx?tab=readme-ov-file)**
- **[Tokio](https://tokio.rs/)**

## Preparing
To start work with notebook you must complete commands below:

1. To get started, clone our repository:
```
git clone https://github.com/Nnenty/lnotebook

cd lnotebook/
```
2. After cloning the repository you need to go to [notebook_example](https://github.com/Nnenty/lnotebook/tree/master/notebook_example) catalog:
```
cd notebook_example/
```
3. Then you should export your database using the command below (change URL on yours):
```
export DATABASE_URL=postgres://username:password@localhost/db`
```
4. Migrate using command:
```
just migrate
```
> **Note**: install [SQLx-cli](https://crates.io/crates/sqlx-cli) if you don't have it installed to run code below.

<h4> You did it! Then you have a database ready to use in notebook. </h4s>

## Start the notebook
> **Clarification**: all code in this section assumes that you have completed all commands from the section [Preparing](https://github.com/Nnenty/lnotebook_api?tab=readme-ov-file#preparing).

- Let's try execute `cargo run` only:
```
cargo run
```
> **Note**: when you use `cargo run` without terminal command, programm should display all total notes.
Read more about terminal commands in [our documentation]().

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
The output of the program will be like this:
```
ID: 1
Name: passwords
Data:
login: krutoy_4el
password: 1234
```

### More about terminal commands
To learn more about commands in terminal similar to `add-note` from the example above read [our documentation]().