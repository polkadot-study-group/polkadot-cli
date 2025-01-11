# URL of the file to download
URL="https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-stable2412/asset_hub_westend_runtime.compact.compressed.wasm"

# Destination file name (optional, you can also use the original file name)
DESTINATION="./nodes/asset_hub_westend_runtime.compact.compressed.wasm"

# Check if the 'nodes' directory exists, if not, create it
if [ ! -d "./nodes" ]; then
    echo "'nodes' directory does not exist. Creating it..."
    mkdir -p "./nodes"
fi

# Use curl to download the file
# curl -o "$DESTINATION" "$URL"
wget -O "$DESTINATION" "$URL"

# Alternatively, you can use wget
# wget -O "$DESTINATION" "$URL"

# Check if the download was successful
if [ $? -eq 0 ]; then
    echo "Download successful: $DESTINATION"
else
    echo "Download failed"
fi