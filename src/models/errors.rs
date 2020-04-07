#[derive(Debug, Serialize, Clone)]
pub struct GraphQlErrorLocation {
    line: i32,
    column: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct GraphQLError {
    message: String,
    locations: Vec<GraphQlErrorLocation>,
}

#[derive(Debug, Serialize, Clone)]
pub struct GraphQLErrors {
    errors: Vec<GraphQLError>,
}

impl GraphQLErrors {
    pub fn new(message: &str) -> GraphQLErrors {
        GraphQLErrors {
            errors: vec![GraphQLError {
                message: message.to_owned(),
                locations: Vec::new(),
            }],
        }
    }
}
