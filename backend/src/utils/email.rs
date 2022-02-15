use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use maud::html;
use std::env;

pub fn send_verification_email(
    target_name: String,
    target_email: String,
    verification_id: String,
) -> Result<String, String> {
    let app_url = env::var("APP_URL").expect("app url not set");
    let body = html! {
        head {
            title {
                "Email Verification"
            }
        }
        div {
            p {
                "Hi " (target_name)
            }
            p {
              "Thanks for getting started with Wecome!"
            }
            p {
              "We need a little more information to complete your registration, including a confirmation of your email address. Click below to confirm your email address:"
            }
            a href=[Some(format!("{}{}",app_url,verification_id))] { "Verification Link" }
            br {}
            p {
              "Best Regards, Wecome Team"
            }
        }
    };

    send_email(
        target_name,
        target_email,
        "Wecome Email Verification".to_string(),
        body.into_string(),
    )
}

fn send_email(
    target_name: String,
    target_email: String,
    subject: String,
    body: String,
) -> Result<String, String> {
    let email = Message::builder()
        .from("Wecome <no-reply@wecome.com>".parse().unwrap())
        .to(format!("{} <{}>", target_name, target_email)
            .parse()
            .unwrap())
        .subject(subject)
        .multipart(
            MultiPart::alternative()
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_PLAIN)
                    .body(String::from("Hello from wecome! If you get this email, it means that wecome email service is failed! Please contact wecome team for more information!"))
            )
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(body)
            )
        )
        .unwrap();

    let email_account = env::var("EMAIL_ACCOUNT").expect("email account not set");
    let password = env::var("EMAIL_PASSWORD").expect("email password not set");

    let creds = Credentials::new(email_account, password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok("Email sent successfully!".to_string()),
        Err(e) => Err(format!("Could not send email: {:?}", e)),
    }
}
