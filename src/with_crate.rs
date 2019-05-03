use validator::ValidationErrors;
use validator::{Validate, ValidationErrorsKind};

#[derive(Debug)]
enum ConsignmentType {
    Incoming,
    Outbound,
}

#[derive(Validate, Debug)]
struct SenderReceiverDetails {
    #[validate(length(max = 3, message = "too long"))]
    business_name: String,
}

#[derive(Validate, Debug)]
struct ContactMethod {
    // TODO: somehow use the enum - not possible to validate enum :(
    #[validate(length(max = 2))]
    method: String,

    #[validate(length(max = 5))]
    value: String,
}

#[derive(Validate, Debug)]
struct ConsignmentData {
    direction: ConsignmentType,

    #[validate(length(max = 3, message = "too long"))]
    who_pays: String,

    #[validate(length(max = 3))]
    cost_centre: String,

    #[validate]
    sender: SenderReceiverDetails,

    #[validate]
    contact_methods: Vec<ContactMethod>,
}

fn print_errors(errors: ValidationErrors, nesting: usize) {
    let prefix = "\t".repeat(nesting);
    for (field, err) in errors.errors() {
        println!("{}Errors on '{}'", prefix, field);
        match err {
            ValidationErrorsKind::Field(all) => {
                for e in all {
                    println!("{}  code={}, message={:?}", prefix, e.code, e.message)
                }
            }
            ValidationErrorsKind::Struct(nested) => print_errors(*nested, nesting + 1),
            ValidationErrorsKind::List(items) => {
                for (index, e) in items {
                    println!("{}[{}]:", prefix, index);
                    print_errors(*e, nesting + 2);
                }
            }
        }
    }
}

fn validate_and_print(cd: ConsignmentData) {
    println!("\n\nObject: {:?}", cd);

    match cd.validate() {
        Ok(data) => print!("Ok: {:?}", data),
        Err(e) => print_errors(e, 0),
    }
}

pub fn play() {
    validate_and_print(ConsignmentData {
        direction: ConsignmentType::Outbound,
        who_pays: "aaaaaa".to_owned(),
        cost_centre: "asd".to_owned(),
        sender: SenderReceiverDetails {
            business_name: "bbbbb".to_owned(),
        },
        contact_methods: vec![
            ContactMethod {
                method: "asd".to_owned(),
                value: "foo".to_owned(),
            },
            ContactMethod {
                method: "asd".to_owned(),
                value: "fooboo".to_owned(),
            },
        ],
    });

    validate_and_print(ConsignmentData {
        direction: ConsignmentType::Incoming,
        who_pays: "aaaaaa".to_owned(),
        cost_centre: "asd".to_owned(),
        sender: SenderReceiverDetails {
            business_name: "bbbbb".to_owned(),
        },
        contact_methods: vec![
            ContactMethod {
                method: "asd".to_owned(),
                value: "foo".to_owned(),
            },
            ContactMethod {
                method: "asd".to_owned(),
                value: "fooboo".to_owned(),
            },
        ],
    });
}
