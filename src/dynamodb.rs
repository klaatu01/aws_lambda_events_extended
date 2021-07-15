use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// The DynamoDBEvent stream event handled to Lambda
// http://docs.aws.amazon.com/lambda/latest/dg/eventsources.html#eventsources-ddb-update
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DynamoDBEvent {
    #[serde(rename = "Records")]
    pub records: Vec<DynamoDBEventRecord>,
}

// DynamoDbEventRecord stores information about each record of a DynamoDb stream event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DynamoDBEventRecord {
    // The region in which the GetRecords request was received.
    #[serde(rename = "awsRegion")]
    pub aws_region: String,

    // The main body of the stream record, containing all of the DynamoDB-specific
    // fields.
    #[serde(rename = "dynamodb")]
    pub dynamodb: DynamoDBStreamRecord,

    // A globally unique identifier for the event that was recorded in this stream
    // record.
    #[serde(rename = "eventID")]
    pub event_id: String,

    // The type of data modification that was performed on the DynamoDB table:
    //
    //    * INSERT - a new item was added to the table.
    //
    //    * MODIFY - one or more of an existing item's attributes were modified.
    //
    //    * REMOVE - the item was deleted from the table
    #[serde(rename = "eventName")]
    pub event_name: DynamoDBOperationType,

    // The AWS service from which the stream record originated. For DynamoDB Streams,
    // this is aws:dynamodb.
    #[serde(rename = "eventSource")]
    pub event_source: String,

    // The version number of the stream record format. This number is updated whenever
    // the structure of Record is modified.
    //
    // Client applications must not assume that eventVersion will remain at a particular
    // value, as this number is subject to change at any time. In general, eventVersion
    // will only increase as the low-level DynamoDB Streams API evolves.
    #[serde(rename = "eventVersion")]
    pub event_version: String,

    // The event source ARN of DynamoDB
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: String,

    // Items that are deleted by the Time to Live process after expiration have
    // the following fields:
    //
    //    * Records[].userIdentity.type
    //
    // "Service"
    //
    //    * Records[].userIdentity.principalId
    //
    // "dynamodb.amazonaws.com"
    #[serde(rename = "userIdentity")]
    pub user_identity: Option<DynamoDBUserIdentity>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DynamoDBUserIdentity {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "principalId")]
    pub principal_id: String,
}

// DynamoDBStreamRecord represents a description of a single data modification that was performed on an item
// in a DynamoDB table.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DynamoDBStreamRecord {
    // The approximate date and time when the stream record was created, in UNIX
    // epoch time (http://www.epochconverter.com/) format.
    #[serde(rename = "ApproximateCreationDateTime")]
    pub approximate_creation_date_time: Option<f64>,

    // The primary key attribute(s) for the DynamoDB item that was modified.
    #[serde(rename = "Keys")]
    pub keys: Option<HashMap<String, AttributeValue>>,

    // The item in the DynamoDB table as it appeared after it was modified.
    #[serde(rename = "NewImage")]
    pub new_image: Option<HashMap<String, AttributeValue>>,

    // The item in the DynamoDB table as it appeared before it was modified.
    #[serde(rename = "OldImage")]
    pub old_image: Option<HashMap<String, AttributeValue>>,

    // The sequence number of the stream record.
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: String,

    // The size of the stream record, in bytes.
    #[serde(rename = "SizeBytes")]
    pub size_bytes: u64,

    // The type of data from the modified DynamoDB item that was captured in this
    // stream record.
    #[serde(rename = "StreamViewType")]
    pub stream_view_type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DynamoDBKeyType {
    Hash,
    Range,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DynamoDBOperationType {
    Insert,
    Modify,
    Remove,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum DynamoDBStreamViewType {
    #[serde(rename = "NEW_IMAGE")]
    NewImage,
    #[serde(rename = "OLD_IMAGE")]
    OldImage,
    #[serde(rename = "NEW_AND_OLD_IMAGE")]
    NewAndOldImage,
    #[serde(rename = "KEYS_ONLY")]
    KeysOnly,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeValue {
    /// <p>An attribute of type Binary. For example:</p> <p> <code>"B": "dGhpcyB0ZXh0IGlzIGJhc2U2NC1lbmNvZGVk"</code> </p>
    #[serde(rename = "B")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<bytes::Bytes>,
    /// <p>An attribute of type Boolean. For example:</p> <p> <code>"BOOL": true</code> </p>
    #[serde(rename = "BOOL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool: Option<bool>,
    /// <p>An attribute of type Binary Set. For example:</p> <p> <code>"BS": ["U3Vubnk=", "UmFpbnk=", "U25vd3k="]</code> </p>
    #[serde(rename = "BS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bs: Option<Vec<bytes::Bytes>>,
    /// <p>An attribute of type List. For example:</p> <p> <code>"L": [ {"S": "Cookies"} , {"S": "Coffee"}, {"N", "3.14159"}]</code> </p>
    #[serde(rename = "L")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l: Option<Vec<AttributeValue>>,
    /// <p>An attribute of type Map. For example:</p> <p> <code>"M": {"Name": {"S": "Joe"}, "Age": {"N": "35"}}</code> </p>
    #[serde(rename = "M")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>An attribute of type Number. For example:</p> <p> <code>"N": "123.45"</code> </p> <p>Numbers are sent across the network to DynamoDB as strings, to maximize compatibility across languages and libraries. However, DynamoDB treats them as number type attributes for mathematical operations.</p>
    #[serde(rename = "N")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    /// <p>An attribute of type Number Set. For example:</p> <p> <code>"NS": ["42.2", "-19", "7.5", "3.14"]</code> </p> <p>Numbers are sent across the network to DynamoDB as strings, to maximize compatibility across languages and libraries. However, DynamoDB treats them as number type attributes for mathematical operations.</p>
    #[serde(rename = "NS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns: Option<Vec<String>>,
    /// <p>An attribute of type Null. For example:</p> <p> <code>"NULL": true</code> </p>
    #[serde(rename = "NULL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,
    /// <p>An attribute of type String. For example:</p> <p> <code>"S": "Hello"</code> </p>
    #[serde(rename = "S")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    /// <p>An attribute of type String Set. For example:</p> <p> <code>"SS": ["Giraffe", "Hippo" ,"Zebra"]</code> </p>
    #[serde(rename = "SS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ss: Option<Vec<String>>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_dynmodb_stream_event() {
        let data = include_bytes!("fixtures/example-dynamo-stream-event.json");
        let parsed: DynamoDBEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: DynamoDBEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
