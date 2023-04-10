# iCountant API
### Rogier Roex PXL
----
#### Steps to run this application
1. Create a .env file in project root folder with db connection values. Copy this into file, but change values for your db:
```
PGHOST=icountant-postgres
PGPORT=5432
PGDATABASE=db_name
PGUSER=user
PGPASSWORD=supersecret

POSTGRES_USER=user
POSTGRES_PASSWORD=supersecret
POSTGRES_DB=db_name
```

2. Have rust + docker + docker-compose installed. Then you can run this API quite easily.
Make sure you are in the project root file (where the Dockerfile is present)
```
cargo clean 
cargo build --release
docker-compose up
```


