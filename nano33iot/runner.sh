#!/bin/bash

# $1 Path to compiled bin

BASENAME=$(basename $1)
DIRNAME=$(dirname $1)
OUTFILE="$DIRNAME/$BASENAME.bin"
OBJCOPY="arm-none-eabi-objcopy"
BOSSAC="~/.arduino15/packages/arduino/tools/bossac/1.7.0-arduino3/bossac"

eval "$OBJCOPY -O binary $1 $OUTFILE"

echo "Double click the button on the Nano 33 IOT to put it in Bootloader mode and press <Enter> to continue."
read

PORT=""
if [[ $2 ]]; then
    PORT=$2
else
    # Find available /dev/ttyACM*
    PORTS=$(ls /dev/ttyACM* 2>/dev/null)
    if [[ -z $PORTS ]]; then
        echo "No serial port found. You may need to specify one yourself like this: 'cargo run -- /dev/ttyACM3'"
        exit 1
    fi
    PORT=$(echo $PORTS | awk '{print $1}')
fi
echo "Using serial port found at $PORT"

eval "$BOSSAC -i -d -U true -i -e -w -v $OUTFILE -R -p $PORT"

