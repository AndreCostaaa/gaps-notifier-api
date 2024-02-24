use webhook::client::{WebhookClient, WebhookResult};

use crate::models::{course_listener::CourseListener, grade::Grade};

pub async fn send_event(listener: &CourseListener, grade: &Grade) -> WebhookResult<()> {
    let client = WebhookClient::new(&listener.webhook_url);
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
