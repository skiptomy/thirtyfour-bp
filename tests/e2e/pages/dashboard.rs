use std::thread;

use thirtyfour::{prelude::ElementQueryable, By, WebDriver};
use tokio::time;

#[allow(dead_code)]
pub async fn dashboard_page(driver: WebDriver) {
    let dashboard_page = driver
        .query(By::XPath("//a[div[@aria-label='Dashboard']]"))
        .first()
        .await
        .unwrap();
    dashboard_page.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);
}
