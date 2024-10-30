use alloy::dyn_abi::EventExt;
use alloy::json_abi::Event;
use alloy::primitives::LogData;
use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;
use web3::types::Log;

#[derive(Clone, Debug)]
pub struct DecodedParam {
    pub name: String,
    pub value: crate::abi::DecodedValue,
}

pub fn decode_log(event: &Event, log: &Log) -> Result<Vec<DecodedParam>> {
    let log_data = log_to_log_data(log);
    let decoded_event = event.decode_log(&log_data, true)?;

    if event.inputs.len() != decoded_event.indexed.len() + decoded_event.body.len() {
        return Err(anyhow!(
            "unexpected number of decoded event inputs; expected {}, got {}",
            event.inputs.len(),
            decoded_event.indexed.len() + decoded_event.body.len(),
        ));
    }

    let decoded_params = decoded_event
        .indexed
        .into_iter()
        .chain(decoded_event.body.into_iter())
        .enumerate()
        .map(|(i, value)| DecodedParam {
            name: event.inputs[i].name.clone(),
            value,
        })
        .collect();

    Ok(decoded_params)
}

fn log_to_log_data(log: &Log) -> LogData {
    let topics = log
        .topics
        .iter()
        .map(|x| x.to_fixed_bytes().into())
        .collect_vec();

    let data = log.data.0.clone().into();

    LogData::new_unchecked(topics, data)
}
