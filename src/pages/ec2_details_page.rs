use std::thread;

use thirtyfour::{prelude::ElementQueryable, By, WebDriver};
use tokio::time;

pub async fn ec2_details_page(driver: WebDriver) {
    let ec2_inventory_page = driver
        .query(By::XPath(
            "//div[@class='ec2InventoryTableScroll']//a",
        ))
        .first()
        .await
        .unwrap();
    ec2_inventory_page.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);
}

pub async fn remediate_window(driver: WebDriver) {
    let remediate_button = driver
        .query(By::XPath(
            "//button[contains(.,'Remediate')]",
        ))
        .first()
        .await
        .unwrap();
    remediate_button.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);

    let remediate_windows = driver
        .query(By::XPath(
            "//div[@class='wrap-1689752436395 container']/div[@class='content']/*/*",
        ))
        .all()
        .await
        .unwrap();

    for remediate_window in remediate_windows {
        let remediate_window_text = remediate_window.text().await.unwrap();
        println!("{:?}", remediate_window_text);
    }
}


// ""
// "Account ID: 257235026455"
// "US East (Ohio)"
// "vol-02da3a15dcd76da4e\nIntegrity Scan\nFile system scan"
// "Description"
// "Trojan.Ransom.AIG was detected in Volume/s (vol-02da3a15dcd76da4e) attached to the instance (i-0f7eb4a797def0e38). Gen:Heur.Ransom.MSIL.!diop!.1 was detected in Volume/s (vol-02da3a15dcd76da4e) attached to the instance (i-0f7eb4a797def0e38). The Brotherhood was detected in Volume/s (vol-02da3a15dcd76da4e) attached to the instance (i-0f7eb4a797def0e38)."
// "Remediation"
// "Stop the instance (i-0f7eb4a797def0e38)\ncontent_copy\nRestore a clean backup to new Instance and Volume/s.\nPerform forensic analysis by restoring to a sandbox or mounting in an isolated environment and patch any security findings."