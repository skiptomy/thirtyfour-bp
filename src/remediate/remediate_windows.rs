use thirtyfour::{prelude::ElementQueryable, By, WebDriver};

pub(crate) async fn general_checks(driver: WebDriver, rp_status: String) {
    let account_id = "161006297526";
    let volume_id = "vol-021f2868c1c963095";
    let region = "US East (Ohio)";
    let account_section = driver
        .query(By::XPath("//div[@class='account-id']"))
        .first()
        .await
        .unwrap();

    assert!(account_section
        .query(By::XPath("p"))
        .first()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .contains(account_id));
    assert_eq!(
        account_section
            .query(By::XPath("span"))
            .first()
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
        region
    );

    let scan_failed_section = driver
        .query(By::XPath("//span[@class='scan failed']"))
        .first()
        .await
        .unwrap();
    let displayed_volume_id = scan_failed_section
        .query(By::XPath("p"))
        .first()
        .await
        .unwrap();
    assert_eq!(displayed_volume_id.text().await.unwrap(), volume_id);

    let scan_links = scan_failed_section
        .query(By::XPath(".//a"))
        .all()
        .await
        .unwrap();
    assert_eq!(scan_links.len(), 2);

    if rp_status == "clean" {
        let remediate_btns = driver
            .query(By::XPath("//div[@class='actions threeItems']/div/button"))
            .all()
            .await
            .unwrap();
        assert_eq!(remediate_btns.len(), 3)
    } else {
        let remediate_btns = driver
            .query(By::XPath("//div[@class='buttons']/button"))
            .all()
            .await
            .unwrap();
        assert_eq!(remediate_btns.len(), 2);
        let not_available_clean = driver
            .query(By::XPath(
                "//div[@class='details error']/div[@class='content']",
            ))
            .first()
            .await
            .unwrap();
        assert_eq!(
            not_available_clean.text().await.unwrap(),
            "Clean backup not available"
        );
    }
}
