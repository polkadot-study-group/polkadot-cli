# To install
This will install and clone the polkadot-sdk repository if the folder did not exist
```bash
$ dot install 
```
# Sart a template node
After dot install you can start a development node with template option
1. minimal template
2. parachain template
3. solocahin template

To run a template node ```$ dot run {template}```
This build and start a node in development environment
```bash
$ dot run solochain
```

# Install the omni-node via polkadot sdk
This will check if polkadot-sdk is available in your directory and build the omni-node template.
a binary file will be generated under nodes/polkadot-omni-node
```bash
$ dot install omni-node
```

To configure your omni node you must specify you chan spec in script/omni-node.sh

# Start the omni-node
```bash
$ dot run omni-node 
```