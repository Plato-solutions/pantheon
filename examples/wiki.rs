/// The example requires to geckodriver have been run

#[cfg(feature = "fantoccini_backend")]
fn main() {
    panic!("This example requires 'fantoccini_backend' feature")
}

#[cfg(feature = "thirtyfour_backend")]
#[tokio::main]
async fn main() {
    use pantheon::{parse, Runner};
    use thirtyfour::{DesiredCapabilities, WebDriver};

    let wiki = std::fs::File::open("examples/wiki.side").unwrap();
    let file = parse(wiki).expect("parsing can't be done...");

    let client = WebDriver::new("http://localhost:4444", DesiredCapabilities::firefox())
        .await
        .expect("can't connect to webdriver");
    let mut runner = Runner::new(&client);
    runner.run(&file.tests[0]).await.unwrap();

    assert_eq!(
        runner.data.get("slogan"),
        Some(&serde_json::Value::String(
            "The Free Encyclopedia".to_owned()
        ))
    );

    runner.close().await.unwrap();
}
