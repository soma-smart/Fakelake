FAKELAKE
=====

Mockup / fake data generator
----------------------------

Fakelake features:
- Very fast
- Easy to use
- Small memory footprint
- Small binary size
- Robust / no unsafe code
- No dependencies
- Cross-platform (Windows, Linux, Mac OS X)
- MIT license

Fakelake is a command line tool that generates fake data from a JSON schema.
It is very fast and can generate millions of rows in seconds.

Fakelake is actively developed and maintained by [SOMA]() in Paris.
Any feedback is welcome!


How to install
--------------

### With cargo

```bash
$ cargo install fakelake
```

### With precompiled binaries

Download the latest release from [here](#)

```bash
$ tar -xvf fakelake-<version>-<platform>.tar.gz
$ cd fakelake-<version>-<platform>
$ ./fakelake --help
```

### From source

```bash
$ git clone
$ cd fakelake
$ cargo build --release
$ ./target/release/fakelake --help
```

How to use
----------

### Simple .parquet file generation with 1 million rows


```bash
$ fakelake --help
$ fakelake --version
$ fakelake generate --help
$ fakelake generate customer.yaml
$ fakelake generate customer.yaml company.yaml
```

Benchmark
---------

Mimesis + Faker + Fakelake



Contributing
------------

Contributions are welcome! Feel free to submit pull requests.

