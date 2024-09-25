mod config;

pub use config;

pub struct Docs {
    config: DocsConfig,
}

impl Docs {
    fn new(config: DocsConfig) -> Self {
        Self { config }
    }

    pub async fn upload_thing(&self, thing: bool) -> anyhow::Result<()> {
        unimplemented!()
    }
}
