use crate::config::Config;
use crate::errors::FakeLakeError;
use crate::rng;

pub trait OutputFormat {
    fn get_extension(&self) -> &str;

    fn generate_file(
        &self,
        file_name: &str,
        config: &Config,
        root_seed: u64,
        file_index: u32,
    ) -> Result<(), FakeLakeError>;

    fn generate_from_config(&self, config: &Config) -> Result<(), FakeLakeError> {
        if config.columns.is_empty() {
            return Err(FakeLakeError::BadYAMLFormat(
                "No columns to generate".to_string(),
            ));
        }

        let default_file_name = config.get_output_file_name(self.get_extension());
        let files = config.get_number_of_generated_files();
        let root_seed = config.resolve_root_seed();

        for f in 0..files {
            let file_name = if files == 1 {
                default_file_name.clone()
            } else {
                format!("{}_{}", default_file_name, f)
            };

            let sub_seed = rng::derive_seed(root_seed, rng::DOMAIN_PROVIDER, &[f as u64]);
            self.generate_file(&file_name, config, sub_seed, f)?;
        }

        Ok(())
    }
}
