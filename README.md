# Build the project
```bash
cargo build --release
```

# To install
This will install and clone the polkadot-sdk repository if the folder did not exist
```bash
$ dot install 
```

# Start the node
This command will start the node using omni-node with westend assethub chain-spec
```bash
$ dot serve
```

# Start a template node (Optional)
An option to run a template node can also be used
1. minimal template
2. parachain template
3. solochain template

To run a template node ```$ dot install --template {template}```
This will compile and start the template node in development mode
```bash
$ dot install --template  solochain
```
