use std::thread;

use thirtyfour::{prelude::ElementQueryable, By, WebDriver};
use tokio::time;

const ACC_ID: &str = "396606860391";
const TEST_RANSOMWARE: &str = "The Brotherhood";
const TEST_MALWARE: &str = "Trojan.Ransom.AIG";

use std::str::FromStr;

pub enum CyberTerm {
    Account,
    Region,
    Ransomware,
    Malware,
    FileSystemIntegrity,
    VulnerabilityType,
}

impl FromStr for CyberTerm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Account" => Ok(CyberTerm::Account),
            "Region" => Ok(CyberTerm::Region),
            "Ransomware" => Ok(CyberTerm::Ransomware),
            "Malware" => Ok(CyberTerm::Malware),
            "File System Integrity" => Ok(CyberTerm::FileSystemIntegrity),
            "Vulnerability Type" => Ok(CyberTerm::VulnerabilityType),
            _ => Err(()),
        }
    }
}

pub async fn check_filters(driver: WebDriver, asset_type: &str, vulnerability_type: &str) {
    disable_all_filters(driver.clone()).await;
    let filters = driver
        .query(By::XPath(
            "//div[@class='filtersScrollRow']/div[not(label)][.//input]",
        ))
        .all()
        .await
        .unwrap();

    for filter in filters {
        filter.click().await.unwrap();
        let sleep_duration = time::Duration::from_secs(2);
        thread::sleep(sleep_duration);

        let filter_name = filter.query(By::XPath(".//input")).first().await.unwrap();
        let filter_name_attr = filter_name.attr("placeholder").await.unwrap().unwrap();
        println!("filter_name_attr: {}", filter_name_attr);

        let term = filter_name_attr.parse::<CyberTerm>().unwrap();

        match term {
            CyberTerm::Account => {
                select_filter(driver.clone(), ACC_ID.to_string()).await;
                done_execute(driver.clone()).await;
            }
            CyberTerm::Region => {
                select_filter(driver.clone(), "US East (Ohio)".to_string()).await;
                done_execute(driver.clone()).await;
            }
            CyberTerm::Ransomware => {
                select_filter(driver.clone(), TEST_RANSOMWARE.to_string()).await;
                done_execute(driver.clone()).await;
                verify_selected_tag(driver.clone(), TEST_RANSOMWARE.to_string()).await;
            }
            CyberTerm::Malware => {
                select_filter(driver.clone(), TEST_MALWARE.to_string()).await;
                done_execute(driver.clone()).await;
                verify_selected_tag(driver.clone(), TEST_MALWARE.to_string()).await;
            }
            CyberTerm::FileSystemIntegrity => {
                let check_status = if asset_type == "ec2" {
                    "Succeeded"
                } else {
                    "Failed"
                };
                select_filter(driver.clone(), check_status.to_string()).await;
                done_execute(driver.clone()).await;
                verify_selected_tag(driver.clone(), check_status.to_string()).await;
            }
            CyberTerm::VulnerabilityType => {
                select_filter(driver.clone(), capitalize_each_word(vulnerability_type)).await;
                done_execute(driver.clone()).await;
                verify_selected_tag(driver.clone(), capitalize_each_word(vulnerability_type)).await;
            }
        }
    }
}

async fn disable_all_filters(driver: WebDriver) {
    let filters_tags = driver
        .query(By::XPath(
            "//div[@class='tags']//div[@class='content']/*[local-name() = 'svg']",
        ))
        .all()
        .await
        .unwrap();

    if !filters_tags.is_empty() {
        for filter_tag in filters_tags {
            filter_tag.click().await.unwrap();
        }
    }
}

async fn select_filter(driver: WebDriver, filter_name: String) {
    let selected_filter = driver
        .query(By::XPath("//div[@class='multipleListOptionText']/div"))
        .with_text(filter_name)
        .first()
        .await
        .unwrap();
    selected_filter.click().await.unwrap();
}

async fn done_execute(driver: WebDriver) {
    let done_btn = driver
        .query(By::XPath("//div[@class='wrapBtnDone']/button"))
        .first()
        .await
        .unwrap();
    done_btn.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(2);
    thread::sleep(sleep_duration);
}

fn capitalize_each_word(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

async fn verify_selected_tag(driver: WebDriver, filter_tag: String) {
    let selected_tag = driver
        .query(By::XPath("//div[@class='tags']//div[@class='content']/div"))
        .with_text(filter_tag.clone())
        .exists()
        .await
        .unwrap();
    assert!(selected_tag, "Selected tag doesn't exist: {}", filter_tag);
}
