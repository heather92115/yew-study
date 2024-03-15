use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};
use crate::sl::gql::{post_gql_query, FetchError};

/// Response JSON wrapper
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseWrapper {
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "getStudyList")]
    pub get_study_list: Vec<Challenge>,
}

/// Represents a challenge presented to a user for vocabulary practice.
///
/// Each challenge is generated based on the user's learning history and targets specific vocabulary
/// that the user is currently studying. It includes a prompt that may consist of a word or phrase
/// in the target language, a sentence for translation, or any other form of query designed to
/// test the user's knowledge and recall of the vocabulary.
///
/// # Fields
///
/// - `vocab_id`: The unique identifier of the vocabulary item being challenged. This relates to a specific
/// word or phrase in the study material.
/// - `vocab_study_id`: The unique identifier for the user's study history with this vocabulary item,
/// allowing for tracking of progress and retrieval of user-specific study data.
/// - `prompt`: The question or challenge presented to the user, designed to elicit the correct response or
/// translation based on the vocabulary being studied.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Challenge {
    #[serde(rename = "vocabId")]
    pub vocab_id: i32,

    #[serde(rename = "vocabStudyId")]
    pub vocab_study_id: i32,
    pub prompt: String,
}

/// Represents a GraphQL query for fetching a list of vocabulary study items.
///
/// This struct is a Rust representation of a GraphQL query defined in the `queries.graphql` file.
/// It uses the `async-graphql` crate to generate Rust types that correspond to the GraphQL schema and query.
/// The query requires two parameters: `awesomeId`, representing the identifier of the awesome person, and
/// `limit`, specifying the maximum number of study items to retrieve.
///
/// The response of this query includes a list of vocabulary study items, each containing the vocabulary
/// identifier (`vocabId`), the vocabulary study identifier (`vocabStudyId`), and a `prompt` for the
/// vocabulary item.
///
/// # Attributes
///
/// - `schema_path`: Path to the GraphQL schema file.
/// - `query_path`: Path to the `.graphql` file containing the query.
/// - `response_derives`: Derive macros for the generated response struct.
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "./graphql/schema.graphql",
query_path = "./graphql/queries.graphql",
response_derives = "Debug"
)]
struct VocabList;

/// Fetches a list of vocabulary study items for a specified user and limit.
///
/// This function creates a GraphQL query to retrieve a list of vocabulary study items
/// associated with the given `awesome_id`. It limits the results to the specified `limit`
/// number of items. The query is serialized to a JSON string and sent to the GraphQL
/// endpoint through the `post_gql_query` function. The function returns the query results
/// as a JSON string or an error if the operation fails.
///
/// # Arguments
///
/// * `awesome_id` - An `i32` representing the unique identifier of the user for whom
///   the vocabulary study list is being fetched.
/// * `limit` - An `i32` that specifies the maximum number of vocabulary study items
///   to be returned.
///
/// # Returns
///
/// A `Result` wrapping a JSON string containing the fetched vocabulary study list
/// on success, or a `FetchError` on failure.
pub async fn fetch_vocab_study_list(awesome_id: i32, limit: i32) -> Result<Vec<Challenge>, FetchError> {
    let build_query = VocabList::build_query(vocab_list::Variables {
        awesome_id: awesome_id.into(),
        limit: limit.into()
    });

    // Serialize the query to a string
    let query_string = serde_json::to_string(&build_query)?;
    let gql_json_res = post_gql_query(query_string).await?;
    let wrapper: ResponseWrapper = serde_json::from_str(&gql_json_res)?;

    Ok(wrapper.data.get_study_list)
}


/// Response JSON wrapper
#[derive(Serialize, Deserialize, Debug)]
pub struct CheckAnswerResponseWrapper {
    pub data: Check,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Check {
    #[serde(rename = "checkResponse")]
    pub response_prompt: String,
}


/// Performs a GraphQL query to check a user's answer against the correct vocabulary answer.
///
/// This function sends a query to a GraphQL server with the user's answer and the associated vocabulary study information.
/// It utilizes the `CheckResponse` GraphQL query which is defined in a separate `.graphql` file.
///
/// ## Parameters:
/// - `answer`: The answer provided by the user.
/// - `challenge`: A `Challenge` struct containing the `vocab_id`, `vocab_study_id`, and the prompt for the vocabulary challenge.
///
/// ## Returns:
/// - A `Result` which is `Ok` containing the server's response prompt if the query was successful, or a `FetchError` if there was an issue with the query.
///
/// ## Example Usage:
/// ```rust
/// let answer = "your_answer".to_string();
/// let challenge = Challenge {
///     vocab_id: 1,
///     vocab_study_id: 1,
///     prompt: "Translate 'hello'".to_string(),
/// };
///
/// if let Ok(response_prompt) = check_vocab_answer(answer, challenge).await {
///     println!("Server response: {}", response_prompt);
/// } else {
///     println!("Error checking the answer.");
/// }
/// ```
///
/// ## Important Notes:
/// - The function constructs a GraphQL query dynamically using the provided answer and challenge details.
/// - The response from the server is deserialized into a `CheckAnswerResponseWrapper` struct to extract the response prompt.
/// - This function is `async` and must be awaited.
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "./graphql/schema.graphql",
query_path = "./graphql/check.graphql",
response_derives = "Debug"
)]
struct CheckResponse;
pub async fn check_vocab_answer(answer: String, challenge: Challenge) -> Result<String, FetchError>{

    let build_query = CheckResponse::build_query(check_response::Variables {
        vocab_id: challenge.vocab_id.into(),
        vocab_study_id: challenge.vocab_study_id.into(),
        entered: answer.clone().into()
    });

    // Serialize the query to a string
    let query_string = serde_json::to_string(&build_query)?;
    let gql_json_res = post_gql_query(query_string).await?;
    let wrapper: CheckAnswerResponseWrapper = serde_json::from_str(&gql_json_res)?;

    Ok(wrapper.data.response_prompt)
}
