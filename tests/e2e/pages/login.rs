use std::thread;

use cucumber::{given, then, when};
use thirtyfour::{prelude::ElementQueryable, By, WebDriver};
use tokio::time;

use crate::world::E2eWorld;

#[given(expr = "I login to the website with credentials - email: {word}, password: {word}")]
#[when(expr = "I login to the website with credentials - email: {word}, password: {word}")]
#[then(expr = "I login to the website with credentials - email: {word}, password: {word}")]
async fn login(world: &mut E2eWorld, email: String, password: String) {
    let driver: &WebDriver = world.context.get().unwrap();
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

    user_email.send_keys(email).await.unwrap();
    user_password.send_keys(password).await.unwrap();

    let submit_btn = driver.query(By::Name("submit")).first().await.unwrap();
    submit_btn.click().await.unwrap();

    let sleep_duration = time::Duration::from_secs(5);
    thread::sleep(sleep_duration);
}
