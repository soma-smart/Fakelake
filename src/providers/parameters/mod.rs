pub mod date;
pub mod datetime;
pub mod f64;
pub mod i32;
pub mod percentage;
pub mod string;
pub mod urange;

use yaml_rust::Yaml;

pub fn get_column_name(column: &Yaml) -> &str {
    column["name"]
        .as_str()
        .unwrap_or_else(|| panic!("Missing column name should not happen !"))
}

#[cfg(test)]
mod tests {
    use super::get_column_name;

    use yaml_rust::YamlLoader;

    // get_column_name
    #[test]
    fn given_yaml_with_column_name_should_return_name() {
        let name_yaml = YamlLoader::load_from_str("name: column_name").unwrap();
        let column_name = get_column_name(&name_yaml[0]);
        assert_eq!(column_name, "column_name");
    }

    #[test]
    #[should_panic]
    fn given_yaml_without_column_name_should_panic() {
        let name_yaml = YamlLoader::load_from_str("").unwrap();
        get_column_name(&name_yaml[0]);
    }
}
