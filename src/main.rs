mod pages;
mod remediate;

use pages::{
    dashboard::dashboard_page,
    ec2_details_page::{details_page, remediate_window},
    filters::check_filters,
    inventory::inventory_page,
    login::elastio_login,
    threats_page::threats_page,
    vul_page::vul_page,
};
use regex::Regex;
use remediate::remediate_windows::general_checks;
use std::{collections::HashMap, time::Duration};
use thirtyfour::prelude::*;

pub const DEFAULT_USER_NAME: &str = "newName";

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--window-size=1920,1080")?;
    let driver = WebDriver::new("http://localhost:9515/", caps).await?;
    let delay = Duration::new(10, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    // login
    elastio_login(driver.clone()).await;

    // dashboard page
    dashboard_page(driver.clone()).await;

    // go to ec2 inventory page
    inventory_page(driver.clone()).await;

    // go to ec2 details page
    details_page(driver.clone(), "ec2".to_string()).await;
    remediate_window(driver.clone()).await;

    // general check for remediate
    general_checks(driver.clone(), "clean".to_string()).await;

    // go to threats page
    // threats_page(driver.clone()).await;

    // go to vul page
    // vul_page(driver.clone()).await;

    // check filters
    // check_filters(driver.clone(), "ec2", "Not Cyber Scanned").await;

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}

async fn disable_all_integrity_scan_options(options: Vec<WebElement>) {
    if !options.is_empty() {
        for option in options {
            option.click().await.unwrap();
        }
    }
}

async fn collect_table_info(driver: WebDriver, table_query: By) -> HashMap<String, String> {
    let table_info_elements = driver.query(table_query).all().await.unwrap();

    let mut table_info: HashMap<String, String> = HashMap::new();

    let rg = Regex::new(r"content_copy").unwrap();

    // // let new_content = file_content.replace("name=\"Feature: ", &modified_feature_name);

    if !table_info_elements.is_empty() {
        for row in table_info_elements {
            let key = row
                .query(By::XPath("div[1] | td[1]"))
                .first()
                .await
                .unwrap();
            let value = row
                .query(By::XPath("div[2] | td[2]"))
                .first()
                .await
                .unwrap();
            let clean_value = rg.replace(&value.text().await.unwrap(), "").to_string();
            if clean_value != "" {
                table_info.insert(key.text().await.unwrap(), clean_value.trim().to_owned());
            }
        }
    }

    table_info
}

async fn collect_custom_table(
    row_1: Vec<WebElement>,
    row_2: Vec<WebElement>,
) -> HashMap<String, String> {
    let zipped_rows = row_1.iter().zip(row_2.iter());
    let mut table_info: HashMap<String, String> = HashMap::new();

    for (row_1, row_2) in zipped_rows {
        let key = row_1.text().await.unwrap();
        let value = row_2.text().await.unwrap();
        table_info.insert(key, value);
    }

    table_info
}

fn format_bytes_to_gb(gigabytes: i64) -> String {
    let bytes = gigabytes as f64 * 1_073_741_824.0;

    let gb = bytes / 1_000_000_000.0;
    format!("{:.2} GB", gb)
}

fn check_rp_status(rp_data: HashMap<String, String>) {
    let rp_status = rp_data.get("Status").unwrap().as_str();
    match rp_status {
        "Not checked" => (),
        "Completed" => {
            let integrity_checks = rp_data.get("Checks").unwrap().as_str();
            let integrity_checks_statuses = integrity_checks.split("\n").collect::<Vec<&str>>();
            let iscan_status = integrity_checks_statuses
                .iter()
                .find(|&x| x.contains("Integrity scan"))
                .unwrap();
            let fs_check_status = integrity_checks_statuses
                .iter()
                .find(|&x| x.contains("Filesystem check"))
                .unwrap();

            let protection_policy_option = "all";
            match protection_policy_option {
                "no" => (),
                "iscan" => {
                    assert!(iscan_status.contains("Passed"));
                    assert!(fs_check_status.contains("Unscanned"));
                }
                "fscheck" => {
                    assert!(iscan_status.contains("Unscanned"));
                    assert!(fs_check_status.contains("Passed"));
                }
                "all" => {
                    assert!(iscan_status.contains("Passed"));
                    assert!(fs_check_status.contains("Passed"));
                }
                _ => panic!("Unknown protection policy option"),
            }
        }
        _ => panic!("Uknown RP status"),
    }
}

async fn check_header(block: WebElement, name: String, header_selector: String) {
    let header = block
        .query(By::XPath(&header_selector))
        .first()
        .await
        .unwrap();
    assert_eq!(header.text().await.unwrap(), name);
}

// async fn check_widget_info(vec_widget: Vec<WebElement>, target_header: String) {
//     for widget in vec_widget {
//         match &target_header {
//             "EBS converage"|"EBS snaps" => {
//                 let size = widget
//                     .query(By::XPath(".//div[@class='size']"))
//                     .first()
//                     .await
//                     .unwrap();
//                 assert_eq!(size.text().await.unwrap(), "8.59 GB");
//             },
//             "EC2 instances" => {
//                 let count = widget.query(by_xpath(".//div[@class='size']"))
//                     .first()
//                     .await
//                     .unwrap();
//                 assert_eq!(count.text().await.unwrap(), "instance count");
//             },
//             _ => ()
//         }
//     }
// }

fn check_assets_widget_headers(vec: Vec<String>, target_headers: String) {
    let header = vec.iter().find(|&x| x.contains(&target_headers)).unwrap();
    assert_eq!(header, &target_headers);
}
