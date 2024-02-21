use futures_util::TryFutureExt;
use mimic;

#[cfg(all(feature = "boring-tls", feature = "boring-tls-native-roots"))]
#[tokio::test]
async fn test_boring_native_roots() {
    let client = mimic::Client::builder()
        .impersonate(mimic::impersonate::Impersonate::Chrome100)
        .build().unwrap();

    let req = client.get("https://badssl.com").send().await.unwrap();

    let text = req.text().await.unwrap();

    assert!(text.contains("<title>badssl.com</title>"));
}
