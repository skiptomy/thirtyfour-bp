use std::thread;

use thirtyfour::{prelude::ElementQueryable, By, WebDriver};
use tokio::time;

pub async fn vul_page(driver: WebDriver) {
    let vul_page = driver
        .query(By::XPath("//a[@class='dashboardHeaderLink infected']"))
        .first()
        .await
        .unwrap();
    vul_page.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);

    let vul_page_all = driver
        .query(By::XPath("//div[contains(@class,'redVariant')]"))
        .first()
        .await
        .unwrap();
    vul_page_all.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);
}
