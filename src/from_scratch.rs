use itertools::Itertools;
use serde::Serialize;
use std::collections::HashMap;
use BadParticipant::*;
use ContactMethod::*;
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShortAlphas {
    TooLong(usize),
    NonAlpha,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum BadParticipant {
    BusinessNameTooLong(usize),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Invalid {
    WhoPays(ShortAlphas),
    CostCentre(ShortAlphas),
    Sender(BadParticipant),
    Receiver(BadParticipant),
    BadPhone(usize), // position
    BadEmail(usize), // position
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

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    contact_methods: HashMap<usize, Vec<String>>,
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

    for (position, cm) in cd.contact_methods.iter().enumerate() {
        match cm {
            Phone(number) => {
                if is_too_long(number, MAX_LEN) {
                    errors.push(BadPhone(position))
                }
            }
            Email(number) => {
                if is_too_long(number, MAX_LEN) {
                    errors.push(BadEmail(position))
                }
            }
        }
    }

    errors
}

fn non_alpha() -> String {
    "should be alphanumeric".to_owned()
}

fn too_long(n: usize) -> String {
    format!("too long (should be less than {} characters)", n)
}

fn upsert(errors: &mut HashMap<usize, Vec<String>>, position: usize, msg: &str) {
    errors
        .entry(position)
        .or_insert(vec![])
        .push(msg.to_owned());
}

fn project_to_object(errors: &Vec<Invalid>) -> ConsignmentFailed {
    let mut failed = ConsignmentFailed {
        who_pays: vec![],
        cost_centre: vec![],
        sender: SenderReceiverFailed {
            business_name: vec![],
        },
        receiver: SenderReceiverFailed {
            business_name: vec![],
        },
        contact_methods: HashMap::new(),
    };

    for &e in errors {
        match e {
            CostCentre(NonAlpha) => failed.cost_centre.push(non_alpha()),
            CostCentre(TooLong(n)) => failed.cost_centre.push(too_long(n)),

            WhoPays(NonAlpha) => failed.who_pays.push(non_alpha()),
            WhoPays(TooLong(n)) => failed.who_pays.push(too_long(n)),

            Sender(BusinessNameTooLong(n)) => failed.sender.business_name.push(too_long(n)),
            Receiver(BusinessNameTooLong(n)) => failed.receiver.business_name.push(too_long(n)),
            BadPhone(position) => upsert(&mut failed.contact_methods, position, "invalid phone"),
            BadEmail(position) => upsert(&mut failed.contact_methods, position, "invalid email"),
        }
    }
    failed
}

fn project_friendly_list(errors: &Vec<Invalid>) -> Vec<String> {
    errors
        .iter()
        .map(|&e| match e {
            CostCentre(NonAlpha) => format!("Cost centre {}", non_alpha()),
            CostCentre(TooLong(n)) => format!("Cost centre is {}", too_long(n)),

            WhoPays(NonAlpha) => format!("Who pays {}", non_alpha()),
            WhoPays(TooLong(n)) => format!("Who pays is {}", too_long(n)),

            Sender(BusinessNameTooLong(n)) => format!("Sender Business Name is {}", too_long(n)),
            Receiver(BusinessNameTooLong(n)) => {
                format!("Receiver Business Name is {}", too_long(n))
            }
            BadPhone(_) => "Contact phone is invalid".to_owned(),
            BadEmail(_) => "Contact email is invalid".to_owned(),
        })
        .dedup()
        .collect()
}

fn validate_and_print(cd: ConsignmentData) {
    println!("\n\n");
    let errors = validate(cd);
    println!(
        "{}",
        serde_json::to_string_pretty(&project_to_object(&errors)).unwrap()
    );

    println!(
        "{}",
        serde_json::to_string_pretty(&project_friendly_list(&errors)).unwrap()
    );
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
        contact_methods: vec![Phone("12345".to_owned())],
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
        contact_methods: vec![Phone("foo".to_owned())],
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
        contact_methods: vec![
            Email("foo".to_owned()),
            Phone("foo".repeat(10).to_owned()),
            Email("foo".to_owned()),
            Phone("foo".repeat(10).to_owned()),
            Email("foo".repeat(10).to_owned()),
        ],
    });
}

#[cfg(test)]
mod test_validation {
    use super::*;

    #[test]
    fn test_valid() {
        let c = ConsignmentData {
            direction: ConsignmentType::Outbound,
            who_pays: "ok".to_owned(),
            cost_centre: "ok".to_owned(),
            sender: SenderReceiverDetails {
                business_name: "ok".to_owned(),
            },
            receiver: SenderReceiverDetails {
                business_name: "ok".to_owned(),
            },
            contact_methods: vec![Phone("12345".to_owned()), Email("asdasd".to_owned())],
        };
        assert!(validate(c).is_empty());
    }

    #[test]
    fn test_flat_fields() {
        let c = ConsignmentData {
            direction: ConsignmentType::Outbound,
            who_pays: "ok".repeat(10),
            cost_centre: "ok".to_owned(),
            sender: SenderReceiverDetails {
                business_name: "ok".to_owned(),
            },
            receiver: SenderReceiverDetails {
                business_name: "ok".to_owned(),
            },
            contact_methods: vec![],
        };
        assert_eq!(vec![WhoPays(TooLong(10)), WhoPays(NonAlpha)], validate(c));
    }

    #[test]
    fn test_contact_methods() {
        let c = ConsignmentData {
            direction: ConsignmentType::Outbound,
            who_pays: "ok".to_owned(),
            cost_centre: "ok".to_owned(),
            sender: SenderReceiverDetails {
                business_name: "ok".to_owned(),
            },
            receiver: SenderReceiverDetails {
                business_name: "ok".to_owned(),
            },
            contact_methods: vec![
                Phone("12345".repeat(10).to_owned()),
                Email("asdasd".repeat(10).to_owned()),
            ],
        };
        assert_eq!(vec![BadPhone(0), BadEmail(1)], validate(c));
    }
}
