## Installation

To install the project, do the following command.

```
$ cargo install
```

## Run API

To run the api, do the following command.

```
$ cargo run
```

## Migration

The migration is handled by prisma orm. Which, in this api, is using the javascript client. To do migration you need to have `npm` installed in your computer. Then create a database structure inside `prisma/schema.prisma`. Then run the following command to create the migration.

```
$ cd migration-helper
$ npx prisma migrate dev
$ cd ..
```

Then it will create the schema in your database. Next, pull the schema definition to the rust API by doing the following command.

```
$ diesel print-schema > src/schema.rs
$ diesel_ext --model > src/models.rs
```
