# RUMQTT full example

This Rust project is a simple collection of binaries that demonstrate the use of the [RUMQTT] crate. The project consists of three binaries:
- broker: A simple MQTT broker that listens for connections on port 1883.
- ponger: A simple MQTT client that subscribes to the topic `ping` and publishes the message `pong` to the topic `pong` whenever it receives a message on the `ping` topic.
- pinger: A simple MQTT client that subscribes to the topic `pong` and publishes the message `ping` to the topic `ping` whenever it receives a message on the `pong` topic.

## Prerequisites

Ensure that you have Rust and Cargo installed on your system. If not, you can install Rust and Cargo by following the instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

## Running the System

1. **Broker:**

   Open a terminal and navigate to the project's root directory. Run the broker using the following command:

   ```bash
   cargo run --bin broker
   ```

2. **Ponger:**

   Open a new terminal and navigate to the project's root directory. Run the ponger using the following command:

   ```bash
   cargo run --bin ponger
   ```

3. **Pinger:**

   Open another terminal and navigate to the project's root directory. Run the pinger using the following command:

   ```bash
   cargo run --bin pinger
   ```

Ensure that you run each program in a separate shell in the order specified above: `broker -> ponger -> pinger`.

The system should now be up and running with the `broker`, `ponger`, and `pinger` processes executing in their respective shells.