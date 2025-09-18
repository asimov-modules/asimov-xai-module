// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

use asimov_module::{
    prelude::*,
    secrecy::{ExposeSecret, SecretString},
    tracing,
};
use core::error::Error;
use serde_json::{Value, json};

#[derive(Clone, Debug, bon::Builder)]
#[builder(on(String, into))]
pub struct Options {
    #[builder(default = "https://api.x.ai")]
    pub endpoint: String,

    #[builder(default = "grok-3-mini")]
    pub model: String,

    pub max_tokens: Option<usize>,

    #[builder(into)]
    pub api_key: SecretString,
}

pub fn generate(input: impl AsRef<str>, options: &Options) -> Result<Vec<String>, Box<dyn Error>> {
    let mut req = json!({
        "model": options.model,
        "input": input.as_ref(),
    });

    if let Some(max_tokens) = options.max_tokens {
        req["max_output_tokens"] = max_tokens.into();
    }

    let mut resp = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .user_agent("asimov-xai-module")
        .build()
        .new_agent()
        .post(format!("{}/v1/responses", options.endpoint))
        .header(
            "Authorization",
            format!("Bearer {}", options.api_key.expose_secret()),
        )
        .header("content-type", "application/json")
        .send_json(&req)
        .inspect_err(|e| tracing::error!("HTTP request failed: {e}"))?;
    tracing::debug!(response = ?resp);

    let status = resp.status();
    tracing::debug!(status = status.to_string());

    let resp: Value = resp
        .body_mut()
        .read_json()
        .inspect_err(|e| tracing::error!("unable to read HTTP response body: {e}"))?;
    tracing::debug!(body = resp.to_string());

    if !status.is_success() {
        tracing::error!("Received an error response: {status}");

        // {
        //   "code": "Client specified an invalid argument",
        //   "error": "Incorrect API key provided: fo***ar. You can obtain an API key from https://console.x.ai."
        // }
        if let Some(message) = resp["error"].as_str() {
            return Err(message.into());
        }
        if let Some(message) = resp.as_str() {
            return Err(message.into());
        }
    }

    let mut responses = Vec::new();

    // {
    //   "created_at": 1758188599,
    //   "id": "...",
    //   "incomplete_details": null,
    //   "max_output_tokens": null,
    //   "metadata": {},
    //   "model": "grok-3-mini",
    //   "object": "response",
    //   "output": [
    //     {
    //       "id": "rs_...",
    //       "status": "completed",
    //       "summary": [
    //         {
    //           "text": "...",
    //           "type": "summary_text"
    //         }
    //       ],
    //       "type": "reasoning"
    //     },
    //     {
    //       "content": [
    //         {
    //           "annotations": [],
    //           "logprobs": null,
    //           "text": "...",
    //           "type": "output_text"
    //         }
    //       ],
    //       "id": "msg_...",
    //       "role": "assistant",
    //       "status": "completed",
    //       "type": "message"
    //     }
    //   ],
    //   "parallel_tool_calls": true,
    //   "previous_response_id": null,
    //   "reasoning": {
    //     "effort": "medium",
    //     "summary": "detailed"
    //   },
    //   "status": "completed",
    //   "store": true,
    //   "temperature": null,
    //   "text": {
    //     "format": {
    //       "type": "text"
    //     }
    //   },
    //   "tool_choice": "auto",
    //   "tools": [],
    //   "top_p": null,
    //   "usage": {
    //     "input_tokens": 8,
    //     "input_tokens_details": {
    //       "cached_tokens": 7
    //     },
    //     "output_tokens": 199,
    //     "output_tokens_details": {
    //       "reasoning_tokens": 188
    //     },
    //     "total_tokens": 207
    //   },
    //   "user": null
    // }

    if let Some(chunks) = resp["output"].as_array() {
        for chunk in chunks {
            if chunk["type"].as_str().is_none_or(|t| t != "message") {
                tracing::debug!("skipping non-message chunk in response: {chunk}");
                continue;
            }
            if chunk["role"].as_str().is_none_or(|r| r != "assistant") {
                tracing::debug!("skipping output chunk not from assistant: {chunk}");
                continue;
            }

            if let Some(chunk_contents) = chunk["content"].as_array() {
                for content in chunk_contents {
                    if content["type"].as_str().is_none_or(|r| r != "output_text") {
                        tracing::debug!("skipping non-text message chunk in response: {chunk}");
                        continue;
                    }

                    if let Some(text) = content["text"].as_str() {
                        responses.push(text.to_string());
                    }
                }
            };

            if let Some(status) = chunk["status"].as_str() {
                tracing::debug!(status);
            }
        }
    }

    Ok(responses)
}
