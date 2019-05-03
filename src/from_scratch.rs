use serde::Serialize;
use BadSernderReceiver::*;
use Invalid::*;
use ShortAlphas::*;

#[derive(Debug)]
enum ConsignmentType {
    Incoming,
    Outbound,
}

#[derive(Debug)]
struct SenderReceiverDetails {
    business_name: String,
}

#[derive(Debug)]
enum ContactMethod {
    Phone(String),
    Email(String),
}

#[derive(Debug)]
struct ConsignmentData {
    direction: ConsignmentType,
    who_pays: String,
    cost_centre: String,
    sender: SenderReceiverDetails,
    receiver: SenderReceiverDetails,
    contact_methods: Vec<ContactMethod>,
}

#[derive(Debug)]
enum ShortAlphas {
    TooLong(usize),
    NonAlpha,
}

#[derive(Debug)]
enum BadSernderReceiver {
    BusinessNameTooLong(usize),
}

#[derive(Debug)]
enum Invalid {
    WhoPays(ShortAlphas),
    CostCentre(ShortAlphas),
    Sender(BadSernderReceiver),
    Receiver(BadSernderReceiver),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SenderReceiverFailed {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    business_name: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConsignmentFailed {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    who_pays: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    cost_centre: Vec<String>,

    sender: SenderReceiverFailed,

    receiver: SenderReceiverFailed,
}

fn is_too_long(s: &String, n: usize) -> bool {
    s.chars().count() > n
}

fn is_non_alpha(s: &String) -> bool {
    s.chars().count() > 3 // TODO: pretend to regex
}

fn validate(cd: ConsignmentData) -> Vec<Invalid> {
    const MAX_LEN: usize = 10;
    let mut errors = vec![];

    if is_too_long(&cd.cost_centre, MAX_LEN) {
        errors.push(CostCentre(TooLong(MAX_LEN)));
    };

    if is_non_alpha(&cd.cost_centre) {
        errors.push(CostCentre(NonAlpha));
    };

    if is_too_long(&cd.who_pays, MAX_LEN) {
        errors.push(WhoPays(TooLong(MAX_LEN)));
    };

    if is_non_alpha(&cd.who_pays) {
        errors.push(WhoPays(NonAlpha));
    };

    if is_too_long(&cd.sender.business_name, MAX_LEN) {
        errors.push(Sender(BusinessNameTooLong(MAX_LEN)));
    };

    if is_too_long(&cd.receiver.business_name, MAX_LEN) {
        errors.push(Receiver(BusinessNameTooLong(MAX_LEN)));
    };

    errors
}

fn non_alpha() -> String {
    "should be alphanumeric".to_owned()
}

fn too_long(n: usize) -> String {
    format!("too long (should be less than {} characters)", n)
}

fn project(errors: Vec<Invalid>) -> ConsignmentFailed {
    let mut failed = ConsignmentFailed {
        who_pays: vec![],
        cost_centre: vec![],
        sender: SenderReceiverFailed {
            business_name: vec![],
        },
        receiver: SenderReceiverFailed {
            business_name: vec![],
        },
    };

    for e in errors {
        match e {
            CostCentre(ShortAlphas::NonAlpha) => failed.cost_centre.push(non_alpha()),
            CostCentre(ShortAlphas::TooLong(n)) => failed.cost_centre.push(too_long(n)),

            WhoPays(ShortAlphas::NonAlpha) => failed.who_pays.push(non_alpha()),
            WhoPays(ShortAlphas::TooLong(n)) => failed.who_pays.push(too_long(n)),

            Sender(BadSernderReceiver::BusinessNameTooLong(n)) => {
                failed.sender.business_name.push(too_long(n))
            }
            Receiver(BadSernderReceiver::BusinessNameTooLong(n)) => {
                failed.receiver.business_name.push(too_long(n))
            }
        }
    }
    failed
}

fn validate_and_print(cd: ConsignmentData) {
    println!("\n\n");
    let failed = project(validate(cd));
    println!("{}", serde_json::to_string_pretty(&failed).unwrap());
}

pub fn play() {
    validate_and_print(ConsignmentData {
        direction: ConsignmentType::Outbound,
        who_pays: "ok".to_owned(),
        cost_centre: "ok".to_owned(),
        sender: SenderReceiverDetails {
            business_name: "ok".to_owned(),
        },
        receiver: SenderReceiverDetails {
            business_name: "ok".to_owned(),
        },
        contact_methods: vec![ContactMethod::Phone("12345".to_owned())],
    });

    validate_and_print(ConsignmentData {
        direction: ConsignmentType::Outbound,
        who_pays: "abc".to_owned(),
        cost_centre: "*^%^&%$^%$".to_owned(),
        sender: SenderReceiverDetails {
            business_name: "ok".to_owned(),
        },
        receiver: SenderReceiverDetails {
            business_name: "ok".to_owned(),
        },
        contact_methods: vec![ContactMethod::Phone("foo".to_owned())],
    });

    validate_and_print(ConsignmentData {
        direction: ConsignmentType::Incoming,
        who_pays: "aaaaaa".to_owned(),
        cost_centre: "asd".to_owned(),
        sender: SenderReceiverDetails {
            business_name: "bbbbb".to_owned(),
        },
        receiver: SenderReceiverDetails {
            business_name: "bbbbb88888888888888888888".to_owned(),
        },
        contact_methods: vec![ContactMethod::Email("foo".to_owned())],
    });
}
