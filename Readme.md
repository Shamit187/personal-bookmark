# Learning Actix

## Attaching routes to a server

There are two beginner way to add any routing to server, using macro and without using macro.

### When using Macro

```rust
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Without Macro

```rust
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## Web Scope

An application's scope acts as a namespace for all routes

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/app")
                .route("/index.html", web::get().to(index)) // /app/index.html
                .route("/index2.html", web::get().to(index2)) // /app/index2.html
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

Why use scope?

- Group
- Guard
- Data

## Too much learning, time for implementation

