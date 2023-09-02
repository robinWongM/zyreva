mod entities;
mod migrator;
mod router;

use axum::{
    error_handling::HandleErrorLayer,
    http::{Method, StatusCode},
    routing::get,
};
use router::{create_router, Context};
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::{prelude::SchemaManager, MigratorTrait};
use std::{
    net::{Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

const DATABASE_URL: &str = "postgres://zyreva:zyreva@db-postgresql:5432/zyreva";

async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let schema_manager = SchemaManager::new(&db);
    migrator::Migrator::refresh(&db).await?;
    assert!(schema_manager.has_table("application").await?);

    Ok(db)
}

#[tokio::main]
async fn main() {
    let db = connect_db().await.expect("Failed to connect to database");
    let db = Arc::new(db);

    let router = create_router().arced();

    #[cfg(all(debug_assertions, not(feature = "k8s")))] // Only export in development builds
    router
        .export_ts(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../packages/rspc/index.ts"))
        .unwrap();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc~'!" }))
        .nest(
            "/rspc",
            router.endpoint(move || Context { db: db.clone() }).axum(),
        )
        .layer(cors)
        .layer(HandleErrorLayer::new(|error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        }));

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl + C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
