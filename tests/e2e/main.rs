mod pages;
mod world;

use futures::FutureExt;
use std::sync::{Arc, Mutex};

use cucumber::{World, codegen::Lazy, WorldInit};
use thirtyfour::{DesiredCapabilities, WebDriver};
use world::{Context, E2eWorld};

static CURRENT_FEATURE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

#[tokio::main]
async fn main() {
    E2eWorld::cucumber()
        .before(move |feature, _, scenario, world| {
            Box::pin(async {
                let mut caps = DesiredCapabilities::chrome();
                caps.add_chrome_arg("--window-size=1920,1080").unwrap();
                let web_driver = WebDriver::new("http://localhost:9515/", caps)
                    .await
                    .unwrap();
                let mut context = Context::new();
                context.insert(web_driver);
                world.context = Arc::new(context);

                println!("scenario_name: {}", scenario.name);

                let mut current_feature = CURRENT_FEATURE.lock().unwrap();
                let feat_name = &feature.name;
                if  feat_name.to_string() != *current_feature {
                    // This is a new feature
                    *current_feature = feature.name.clone();
                    println!("Before feature: {}", current_feature);
                    // Additional setup for new feature...
                }
            })
        })
        .max_concurrent_scenarios(1)
        .fail_on_skipped()
        .after(move |_, _, _, world| {
            async move {
                if let Some(world) = world {
                    // End the webdriver session and close the browser.
                    let web_driver: &WebDriver = world.context.get().unwrap();
                    web_driver.clone().quit().await.unwrap();
                }
            }
            .boxed_local()
        })
        .run_and_exit("./tests/e2e/features/")
        .await;
}
