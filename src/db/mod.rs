use surrealdb::engine::remote::ws;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use valence::log::warn;
use valence::prelude::*;

pub mod player;

#[derive(Resource)]
pub struct DB {
    pub(crate) runtime: tokio::runtime::Runtime,
    pub(crate) db: Option<Surreal<ws::Client>>,
}

impl DB {
    // use tokio runtime to execute async code
    pub(crate) fn block_on<F: std::future::Future>(&self, f: F) -> F::Output {
        self.runtime.block_on(f)
    }

    pub(crate) fn connect(&mut self, force: bool) -> surrealdb::Result<()> {
        if force || self.db.is_none() {
            return self.runtime.block_on(async {
                self.db = Some(Surreal::new::<Ws>("127.0.0.1:8000").await?);

                // Signin as a namespace, database, or root user
                self.db
                    .as_ref()
                    .unwrap()
                    .signin(Root {
                        username: "root",
                        password: "root",
                    })
                    .await?;

                // Select a specific namespace / database
                self.db
                    .as_ref()
                    .unwrap()
                    .use_ns("minecraft")
                    .use_db("server")
                    .await?;

                Ok(())
            });
        }
        Ok(())
    }

    pub fn get_db(&mut self) -> surrealdb::Result<&Surreal<ws::Client>> {
        self.connect(false)?;
        Ok(self.db.as_ref().unwrap())
    }

    // try restarting the connection
    pub(crate) fn handle_error(&mut self, err: surrealdb::Error) -> surrealdb::Result<()> {
        warn!("Db Error: {:?}", err);
        warn!("Attempting to restart connection...");

        for i in 0..5 {
            warn!("Attempt {}...", i + 1);
            if let Ok(_) = self.connect(true) {
                return Ok(());
            }
        }

        Err(err)
    }
}