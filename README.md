<h1 align ="center">LNotebook API</h1>
<div align ="center">
<strong>
<h4> asynchronous API to creating notebooks that stores notes in a database </h4>
</strong>
API that will helps you quickly make your own notebook
</div>

This API using:
- **[SQLx](https://github.com/launchbadge/sqlx?tab=readme-ov-file)**
- **[Tokio](https://tokio.rs/)** 

## Preparing
1. To get started, run the command below; it will clones this repository into your current directory:
```
git clone https://github.com/Nnenty/lnotebook_api

cd lnotebook_api/
```
2. After cloning the repository you need to go to `notebook_example` catalog to run notebook:
```
cd notebook_example/
```
3. Then you should export your database using the command:
(change fields on yours):
```
export DATABASE_URL=postgres://username:password@localhost/db`
```
4. Migrate your db using command:
```
just migrate
```
 
You did it! Then you have a database ready to use in notebook.