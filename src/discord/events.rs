use webhook::client::{WebhookClient, WebhookResult};

use crate::models::grade::Grade;

pub async fn send_event(webhook_url: &str, grade: &Grade) -> WebhookResult<()> {
    let client = WebhookClient::new(&webhook_url);
    let mention = ""; //"@here";
    client
        .send(|message| {
            message.content(&*format!(
                "{} [{}-{}] {} \nMoyenne: {:.1}",
                mention, grade.course, grade.class, grade.name, grade.class_average
            ))
        })
        .await?;

    Ok(())
}
