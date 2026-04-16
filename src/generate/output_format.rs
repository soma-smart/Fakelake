use crate::config::Config;
use crate::errors::FakeLakeError;
use crate::rng;

pub trait OutputFormat {
    fn get_extension(&self) -> &str;

    fn generate_file(
        &self,
        file_name: &str,
        config: &Config,
        file_seed: u64,
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
        let seed_was_provided = config.info.as_ref().and_then(|i| i.seed).is_some();
        if !seed_was_provided {
            println!(
                "No seed specified — using random seed: {} (add 'seed: {}' to your config to reproduce this run)",
                root_seed, root_seed
            );
        }

        let extension = self.get_extension();

        for f in 0..files {
            let file_name = if files == 1 {
                default_file_name.clone()
            } else {
                match default_file_name.rfind(extension) {
                    Some(pos) if !extension.is_empty() => {
                        format!("{}_{}{}", &default_file_name[..pos], f, extension)
                    }
                    _ => format!("{}_{}", default_file_name, f),
                }
            };

            let file_seed = rng::derive_seed(root_seed, rng::DOMAIN_FILE, &[f as u64]);
            self.generate_file(&file_name, config, file_seed)?;
        }

        Ok(())
    }
}
