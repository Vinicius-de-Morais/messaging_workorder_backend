use sea_orm::*;

use sea_orm::ConnectOptions;

const DATABASE_URL: &str = "postgres://postgres:1234@postgres:5432/postgres";

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(DATABASE_URL.to_owned());
    opt.max_connections(10) // Limite de conexões simultâneas
        .min_connections(1) // Número mínimo de conexões no pool
        .connect_timeout(std::time::Duration::from_secs(30)) // Tempo limite de conexão
        .idle_timeout(std::time::Duration::from_secs(600)); // Tempo de inatividade antes de encerrar conexões

    let db = Database::connect(opt).await?;
    Ok(db)
}
