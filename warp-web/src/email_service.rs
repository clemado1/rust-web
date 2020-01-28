extern crate lettre;
extern crate lettre_email;

use crate::errors::ServiceError;
use crate::models::Invitation;
use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

pub fn send_invitation(invitation: &Invitation) -> Result<(), ServiceError> {
    let smtp_address = "smtp.gmail.com";
    let gmail_id =
        std::env::var("GMAIL_ID").expect("Email address must be set");
    let gmail_pw = std::env::var("GMAIL_PW").expect("Password must be set");

    // recipient from the invitation email
    let recipient = invitation.email.as_str();

    let email_body = format!(
        "Please click on the link below to complete registration. <br />
        <a href=\"http://localhost:3000/register.html?id={}&email={}\">
        http://localhost:3030/register</a><br>
        your Invitation expires on <strong>{}</strong>",
        invitation.id,
        invitation.email,
        invitation
            .expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    );

    let email = EmailBuilder::new()
        .to(recipient)
        .from(gmail_id.as_str())
        .subject("You have been invited to join Simple-Auth-Server Rust")
        .html(email_body.to_string())
        .build()
        .unwrap()
        .into();

    let creds = (gmail_id, gmail_pw).into_credentials();

    let mut client = SmtpClient::new_simple(smtp_address)
        .unwrap()
        .credentials(creds)
        .transport();
    let result = client.send(email);

    match result {
        Ok(_) => {
            println!("Email sent: \n {:#?}", result);
            Ok(())
        }
        Err(error) => {
            println!("Send Email Error: \n {:#?}", error);
            Err(ServiceError::InternalServerError)
        }
    }
}
