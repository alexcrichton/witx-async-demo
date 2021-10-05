use std::time::SystemTime;

witx_bindgen_rust::import!("../imports.witx");
witx_bindgen_rust::export!("../exports.witx");

struct Exports;

#[witx_bindgen_rust::async_trait(?Send)]
impl exports::Exports for Exports {
    async fn run() {
        println!("current time is {:?}", SystemTime::now());
        // ...
    }
}
