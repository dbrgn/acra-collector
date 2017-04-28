//! ACRA Collector
//! 
//! A simple collector for ACRA that does the following:
//! 
//! - Append report to `crashes.txt` file
//! - Send an e-mail with the stack trace
extern crate env_logger;
extern crate iron;
extern crate lettre;
extern crate router;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use iron::prelude::*;
use iron::{middleware, status};
use lettre::email::EmailBuilder;
use lettre::transport::EmailTransport;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};
use router::Router;
use serde_json::Value;

type Object = HashMap<String, Value>;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    host: String,
    port: u16,
    email_from: String,
    email_to: String,
    smtp_host: String,
    smtp_port: u16,
    smtp_user: String,
    smtp_pass: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct Report {
    REPORT_ID: String,

    ANDROID_VERSION: String,
    APPLICATION_LOG: Option<String>,
    APP_VERSION_CODE: u64,
    APP_VERSION_NAME: String,
    AVAILABLE_MEM_SIZE: u64,
    BRAND: String,
    BUILD: Object,
    BUILD_CONFIG: Option<Object>,
    CRASH_CONFIGURATION: Object,
    CUSTOM_DATA: Object,
    DEVICE_FEATURES: Object,
    DEVICE_ID: Option<String>,
    DISPLAY: Object,
    DROPBOX: Option<String>,
    DUMPSYS_MEMINFO: String,
    ENVIRONMENT: Object,
    EVENTSLOG: Option<String>,
    FILE_PATH: String,
    INITIAL_CONFIGURATION: Object,
    INSTALLATION_ID: String,
    IS_SILENT: bool,
    LOGCAT: String,
    MEDIA_CODEC_LIST: Option<Object>,
    PACKAGE_NAME: String,
    PHONE_MODEL: String,
    PRODUCT: String,
    RADIOLOG: Option<String>,
    SETTINGS_GLOBAL: Option<Object>,
    SETTINGS_SECURE: Option<Object>,
    SETTINGS_SYSTEM: Option<Object>,
    SHARED_PREFERENCES: Object,
    STACK_TRACE: String,
    THREAD_DETAILS: Option<String>,
    TOTAL_MEM_SIZE: u64,
    USER_APP_START_DATE: String,
    USER_COMMENT: Option<String>,
    USER_CRASH_DATE: String,
    USER_EMAIL: String,
}

struct ReportHandler {
    pub config: Config,
}

impl middleware::Handler for ReportHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        println!("Incoming report...");

        // Get raw body
        let mut payload = String::new();
        match req.body.read_to_string(&mut payload) {
            Ok(_) => {},
            Err(e) => {
                println!("Error: Could not read body to string: {}", e);
                return Ok(Response::with(status::InternalServerError));
            },
        };

        // Store report to file
        match OpenOptions::new()
                          .create(true)
                          .append(true)
                          .open("crashes.txt") {
            Ok(mut file) => if let Err(e) = writeln!(file, "{}", &payload) {
                println!("Error: Could not write crash to file: {:?}", e);
                return Ok(Response::with(status::InternalServerError));
            },
            Err(e) => {
                println!("Error: Could not open crash log file: {:?}", e);
                return Ok(Response::with(status::InternalServerError));
            }
        };
        println!("  -> Saved to crash log file");

        // Parse report
        let report: Report = match serde_json::from_str(&payload) {
            Ok(r) => r,
            Err(e) => {
                println!("Could not parse report: {:?}", e);
                return Ok(Response::with(status::InternalServerError));
            },
        };
        println!("  -> Parsed report");
        println!("  -> Report ID is {}", report.REPORT_ID);

        // Create and send e-mail
        let email_option = EmailBuilder::new()
            .to(&*self.config.email_to)
            .from(&*self.config.email_from)
            .subject(&format!("New crash of {} ({})", report.PACKAGE_NAME, report.APP_VERSION_NAME))
            .text(&format!("A new crash happened:\r\n\r\n- Report ID: {}\r\n- Version: {} ({})\r\n- Android version: {}\r\n\r\nStack trace:\r\n\r\n{}",
                           report.REPORT_ID, report.APP_VERSION_NAME, report.APP_VERSION_CODE, report.ANDROID_VERSION, report.STACK_TRACE))
            .build();
        match email_option {
            Ok(email) => {
                match SmtpTransportBuilder::new((&*self.config.smtp_host, self.config.smtp_port))
                        .map(|t| t.credentials(&*self.config.smtp_user, &*self.config.smtp_pass))
                        .map(|t| t.security_level(SecurityLevel::AlwaysEncrypt))
                        .map(|t| t.smtp_utf8(true))
                        .map(|t| t.connection_reuse(true))
                        .map(|t| t.build()) {
                    Ok(mut mailer) => match mailer.send(email) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("Could not send email: {:?}", e);
                            return Ok(Response::with(status::InternalServerError));
                        },
                    },
                    Err(e) => {
                        println!("Could not connect to SMTP server: {:?}", e);
                        return Ok(Response::with(status::InternalServerError));
                    }
                };
            },
            Err(e) => {
                println!("Could not prepare email: {:?}", e);
                return Ok(Response::with(status::InternalServerError));
            }
        }
        println!("  -> Sent report e-mail");

        Ok(Response::with(status::Ok))
    }
}

fn main() {
    env_logger::init().unwrap();

    // Load config
    let file = File::open("config.json").unwrap();
    let config: Config = serde_json::from_reader(&file).unwrap();

    let mut router = Router::new();
    router.post("/report", ReportHandler { config: config.clone() }, "report");
    println!("Starting server on {}:{}...", config.host, config.port);
    Iron::new(router).http(&format!("{}:{}", config.host, config.port)).unwrap();
}
