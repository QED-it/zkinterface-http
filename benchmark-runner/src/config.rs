
use serde_yaml;

#[bench]
fn bench1(b: &mut Bencher) {
    let config = read_to_string("config.yaml").unwrap();
    let config: Vec<String> = serde_yaml::from_str(&config).unwrap();

    for command in &config {
        println!("{:?}", command);
        let res = run(b, command);

        match res {
            Ok(()) => println!("Done."),
            Err(err) => println!("Error: {}", err),
        }
    }
}
