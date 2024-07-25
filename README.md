<h1 align ="center">LNotebook API</h1>
<div align ="center">
<h3> asynchronous API to creating notebooks that stores notes in a database </h3>

<h5> API that will helps you quickly make your own notebook and commands to manipulate it </h5>
</div>

<h3>This API using:</h3>

- **[SQLx](https://github.com/launchbadge/sqlx?tab=readme-ov-file)**
- **[Tokio](https://tokio.rs/)**

## Preparing
To start work with notebook you must complete commands below:

1. To get started, run the command below; it will clones this repository into your current directory:
```
git clone https://github.com/Nnenty/lnotebook_api

cd lnotebook_api/
```
2. After cloning the repository you need to go to `notebook_example` catalog to run notebook:
```
cd notebook_example/
```
3. Then you should export your database using the command (change URL on yours):
```
export DATABASE_URL=postgres://username:password@localhost/db`
```
4. Migrate using command:
> Note: install [SQLx-cli](https://crates.io/crates/sqlx-cli) if you don't have it installed to run code below.
```
just migrate
```

<h4> You did it! Then you have a database ready to use in notebook. </h4s>