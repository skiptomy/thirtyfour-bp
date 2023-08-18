use std::thread;

use thirtyfour::{prelude::ElementQueryable, By, WebDriver};
use tokio::time;

#[allow(dead_code)]
pub async fn inventory_page(driver: WebDriver) {
    let ec2_inventory_page = driver
        .query(By::XPath(
            "//div[@class='wrapTotalInventoryItemsBlock']/a[contains(.,'EBS')]",
        ))
        .first()
        .await
        .unwrap();
    ec2_inventory_page.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);
}
