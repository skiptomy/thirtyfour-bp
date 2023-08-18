use std::thread;

use thirtyfour::{prelude::ElementQueryable, By, WebDriver};
use tokio::time;

#[allow(dead_code)]
pub async fn details_page(driver: WebDriver, asset_type: String) {
    let details_page_path = if asset_type == "ec2" {
        "//div[@class='ec2InventoryTableScroll']//a"
    } else {
        "//div[@class='ebsInventoryTableScroll']//a"
    };
    let ec2_inventory_page = driver
        .query(By::XPath(details_page_path))
        .first()
        .await
        .unwrap();
    ec2_inventory_page.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);

    let vul_tab = driver
        .query(By::XPath("//button[contains(@class,'jsVcVulnerabilitiesTab')]"))
        .first()
        .await
        .unwrap();
    vul_tab.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);
}

#[allow(dead_code)]
pub async fn remediate_window(driver: WebDriver) {
    let remediate_button = driver
        .query(By::XPath("//button[contains(.,'Remediate')]"))
        .first()
        .await
        .unwrap();
    remediate_button.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);
}
