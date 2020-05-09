# About

This repository contains sample code for you to get started with the Channels application in IOTA Streams.

You can find documentation for these examples on the [IOTA documentation portal](https://docs.iota.org/docs/channels/introduction/get-started.md).

# Goal
The goal of this example is to show how a High-level API can reduce development complexity when working with IOTA-channels.
The two classes `channel_lite_writer` and `channel_lite_reader` fix the Tangle as transport medium for the stream, removing the need for the developer to set variables such as Minimum Weight Maginitude. 

# How It Works

Use `channel.open()` to open the channel and get the announcement verifier <br />
Use `channel.write_signed()` to write signed a message(public or masked) into the channel <br />
Use `channel.write_tagged()` to write tagged a message(public or masked) into the channel <br />
<br />
Use `channel.connect()` to connect to a channel<br />
Use `channel.read_signed()` to read signed a message from the channel<br />
Use `channel.read_tagged()` to read tagged a message from the channel<br />
Use `channel.disconnect()` to disconnect from a channel<br />

# Try it yourself
Clone the repo: <br />
`git clone https://github.com/AleBuser/iota-channels-lite`<br />
Open the Folder:<br />
`cd iota-channels-lite`<br />
Run the code:<br />
`cargo run`<br />

# Todo
- Fix "Link not found" error <br />
- Implement remove Subscriber <br />
