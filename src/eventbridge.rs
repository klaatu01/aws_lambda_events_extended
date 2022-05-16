use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct EventBridgeEvent<T = Value> {
    pub id: String,

    pub version: String,

    pub account: String,

    pub time: String,

    pub region: String,

    pub resources: Vec<String>,

    pub source: String,

    #[serde(rename = "detail-type")]
    pub detail_type: String,

    pub detail: T,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[derive(Debug, Deserialize, Serialize, std::cmp::PartialEq)]
    struct EC2StateChangeDetail {
        #[serde(rename = "instance-id")]
        pub instance_id: String,
        pub state: String,
    }

    #[test]
    fn example_event_bridge_event() {
        let data = include_str!("fixtures/example-event-bridge-event.json");
        let parsed: EventBridgeEvent<EC2StateChangeDetail> = serde_json::from_str(data).unwrap();
        assert_eq!(parsed.version, "0");
        assert_eq!(parsed.id, "6a7e8feb-b491-4cf7-a9f1-bf3703467718");
        assert_eq!(parsed.detail_type, "EC2 Instance State-change Notification");
        assert_eq!(parsed.source, "aws.ec2");
        assert_eq!(parsed.time, "2017-12-22T18:43:48Z");
        assert_eq!(parsed.region, "us-west-1");
        assert_eq!(
            parsed.resources,
            vec!["arn:aws:ec2:us-west-1:123456789012:instance/i-1234567890abcdef0"]
        );
        assert_eq!(
            parsed.detail,
            EC2StateChangeDetail {
                instance_id: "i-1234567890abcdef0".to_string(),
                state: "terminated".to_string()
            }
        )
    }
}
