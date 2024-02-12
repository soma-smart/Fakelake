Generate data
---------------

Now that you have a yaml file ready to be used (cf [here](create_your_yaml_file.md)), we can generate it using fakelake.

### Get executable
##### With precompiled binaries

Download the latest release from [here](https://github.com/soma-smart/Fakelake/releases)

```bash
tar -xvf Fakelake_<version>_<target>.tar.gz
./fakelake --help
```

##### From source
```bash
git clone https://github.com/soma-smart/Fakelake
cd fakelake
cargo build --release
./target/release/fakelake --help
```

### Generate
To generate from one YAML file you can use:
```bash
fakelake generate config_file.yaml
```

You can also chain the files to generate multiples:
```bash
fakelake generate first_file.yaml second_file.yaml
```