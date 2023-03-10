# Event Processor Rust

## Objective

A simple event processor that can read events from Rudderstack and push to BigQuery.

What are we doing and why? What problem will this solve? What are the goals and
non-goals? This is your executive summary; keep it short, elaborate below.

## Simple design

- [] Recreate traffic to publish JSON a la Rudderstack webhook
    - [] Use locust/python to push events to endpoint
- [x] Create an endpoint to which the webhook has to publish events
    - At the endpoint, create a producer/input connector for Fluvio that parses input JSON and publishes to a Fluvio topic
- AVRO Schema validation can be done via a SmartModule - WASM filter
- JSON-to-AVRO transformation also done via a SmartModule - WASM filter
- Create a consumer/output (sink) connector that reads from the topic and writes to BigQuery
