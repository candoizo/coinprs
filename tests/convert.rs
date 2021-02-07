#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}


#[path = "../src/config.rs"]
mod conf;

#[test]
fn it_works_2() {
    // assert_eq!(2 + 2, 4);

    let tint = conf::parse_tint(&"#FFFFFF".to_owned());
    assert_eq!(comfy_table::Color::White, tint);

}
