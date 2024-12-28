# Running project

```bash
    docker-compose up --build
```

## Running project manualy

First init the postgree to dev propose

```bash
docker run -p 5432:5432 -v /tmp/database:/var/lib/postgresql/data -e POSTGRES_PASSWORD=1234 -d postgres
```


Then run

```bash
docker build -t messaging_workorder_backend .
docker run messaging_workorder_backend
```

or

```bash
docker build -t messaging_workorder_backend .
docker run -p 8000:8000 --rm --name messaging_workorder_backend1 messaging_workorder_backend
```

## Database Dev Environment

First is needed to download the postgres image

```bash
    docker pull postgres
```

Then to start the container properly just run

```bash
    docker run -p 5432:5432 -v /tmp/database:/var/lib/postgresql/data -e POSTGRES_PASSWORD=1234 -d postgres
```