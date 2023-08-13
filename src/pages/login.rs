use std::thread;

use thirtyfour::{prelude::ElementQueryable, By, WebDriver};
use tokio::time;

pub async fn elastio_login(driver: WebDriver) {
    driver.goto("http://localhost:3201/e2e-host").await.unwrap();

    let elem_logout = driver
        .query(By::ClassName("jsLogoutLink"))
        .first()
        .await
        .unwrap();
    elem_logout.click().await.unwrap();

    let user_email = driver
        .query(By::Name("email"))
        .and_clickable()
        .first()
        .await
        .unwrap();
    let user_password = driver
        .query(By::Name("password"))
        .and_clickable()
        .first()
        .await
        .unwrap();

    //  // let sleep_duration = time::Duration::from_secs(10);
    //  // thread::sleep(sleep_duration);

    user_email.send_keys("testbs@elastio.com").await.unwrap();
    user_password.send_keys("GodBless21Year@").await.unwrap();

    let submit_btn = driver.query(By::Name("submit")).first().await.unwrap();
    submit_btn.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);
}
