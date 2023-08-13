use std::thread;

use thirtyfour::{prelude::ElementQueryable, By, WebDriver};
use tokio::time;

pub async fn inventory_page(driver: WebDriver) {
    let ec2_inventory_page = driver
        .query(By::XPath(
            "//div[@class='wrapTotalInventoryItemsBlock']/a[contains(.,'EC2')]",
        ))
        .first()
        .await
        .unwrap();
    ec2_inventory_page.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);
}
