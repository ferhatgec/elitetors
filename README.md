# [elite](https://github.com/ferhatgec/elite)tors
## [elite](https://github.com/ferhatgec/elite) -> rust converter

### input:
```rs
required_version is 0.1

set ProjectName as "elitetors"
set HOME        as env "HOME"


for argument "install" [
    use exec "cargo install --path ."

    for exists "{HOME}.cargo/bin/{ProjectName}" [
        println "{ProjectName} installed to {HOME}.cargo/bin/{ProjectName}."
    ]

    use signal "exit"
]
```

### output
```py
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_must_use)]
#[allow(non_snake_case)]
fn main() {
    let cli_arguments: Vec<_> = std::env::args().collect();
if "0.1" != "0.1"
{
    println!("elite: Required higher version");
std::process::exit(1);
}
let mut ProjectName = "elitetors";
let mut HOME = "/home/gech";
if cli_arguments.len() >= 2 && cli_arguments.last().unwrap() == "install"
{
    systemf!("cargo install --path .");
if std::path::Path::new("/home/gech/.cargo/bin/elitetors").exists()
{
    println!("elitetors installed to /home/gech/.cargo/bin/elitetors.");
}
std::process::exit(1);
}
}

```

### elitetors licensed under the terms of MIT License
