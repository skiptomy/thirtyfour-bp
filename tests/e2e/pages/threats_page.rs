use std::thread;

use thirtyfour::{prelude::ElementQueryable, By, WebDriver};
use tokio::time;

#[allow(dead_code)]
pub async fn threats_page(driver: WebDriver) {
    let threats_page = driver
        .query(By::XPath(
            "//a[@class='dashboardHeaderLink threats infected']",
        ))
        .first()
        .await
        .unwrap();
    threats_page.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);

    let threats_page_all = driver
        .query(By::XPath("//div[@class='blockHorizontalThreeItems']/div[contains(.,'Active Ransomware')]//a[span]"))
        .first()
        .await
        .unwrap();
    threats_page_all.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);
}
