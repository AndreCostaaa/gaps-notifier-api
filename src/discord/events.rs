use webhook::client::{WebhookClient, WebhookResult};

use crate::models::{course_listener::CourseListener, grade::Grade};

pub async fn send_event(listener: &CourseListener, grade: &Grade) -> WebhookResult<()> {
    let client = WebhookClient::new(&listener.webhook_url);

    client.send(|message| message
        .content(&*format!("@here [{}] {} \nmoyenne: {:.1}", grade.class, grade.name, grade.class_average))
    ).await?;

    Ok(())
}
