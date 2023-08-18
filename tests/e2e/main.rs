mod pages;
mod world;

use futures::FutureExt;
use std::sync::Arc;

use cucumber::World;
use thirtyfour::{DesiredCapabilities, WebDriver};
use world::{Context, E2eWorld};

#[tokio::main]
async fn main() {
    E2eWorld::cucumber()
        .before(move |_, _, _, world| {
            Box::pin(async {
                let mut caps = DesiredCapabilities::chrome();
                caps.add_chrome_arg("--window-size=1920,1080").unwrap();
                let web_driver = WebDriver::new("http://localhost:9515/", caps)
                    .await
                    .unwrap();
                let mut context = Context::new();
                context.insert(web_driver);
                world.context = Arc::new(context);
            })
        })
        .max_concurrent_scenarios(1)
        .fail_on_skipped()
        .after(move |_, _, _, _, world| {
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
