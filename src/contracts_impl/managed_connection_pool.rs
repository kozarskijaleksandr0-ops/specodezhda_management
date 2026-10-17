use async_sqlite::{Client, ClientBuilder};

type ManagedPool = (
    smol::channel::Sender<Client>,
    smol::channel::Receiver<Client>,
);

pub struct ManagedConnectionPool {
    database_path: String,
    max_connections_count: usize,
    pool: ManagedPool,
}

impl ManagedConnectionPool {
    pub async fn initialize(database_path: String, max_connections_count: usize) -> Self {
        let path_clone = database_path.clone();
        let (sender, receiver) = smol::channel::bounded::<Client>(max_connections_count);

        for _ in 0..max_connections_count {
            let connection = Self::create_connection(path_clone.clone()).await;
            sender
                .send(connection)
                .await
                .expect("Не удается наполнить пул соединений.");
        }

        Self {
            database_path: path_clone,
            max_connections_count,
            pool: (sender, receiver),
        }
    }

    async fn create_connection(database_path: String) -> Client {
        let builder: ClientBuilder = ClientBuilder::new().path(database_path.clone());
        builder
            .open()
            .await
            .expect("Не удается открыть соединение.")
    }

    pub async fn acquire_connection(&self) -> Result<Client, Box<dyn std::error::Error>> {
        let connection = self.pool.1.recv().await?;
        Ok(connection)
    }

    pub async fn return_connection(
        &self,
        connection: Client,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.pool.0.send(connection).await?;
        Ok(())
    }

    pub fn database_path(&self) -> &str {
        &self.database_path
    }

    pub fn max_connections_count(&self) -> usize {
        self.max_connections_count
    }
}