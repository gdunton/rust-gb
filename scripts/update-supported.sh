#!/bin/bash -eu

cd ../docs

SUPPORTED_CODES=$(cat ../src/gameboy/opcodes/opcodes.rs \
    | grep -E '0x[0-9A-F][0-9A-F] =>' \
    | sed 's/[[:space:]]*\(0x[0-9A-F][0-9A-F]\).*/"\1",/g')

# Remove the last ,
SUPPORTED_CODES="${SUPPORTED_CODES%?}"

# Put into a single line
SUPPORTED_CODES=$(echo $SUPPORTED_CODES)

CB_CODES=$(cat ../src/gameboy/opcodes/cb_opcodes.rs \
    | grep -E '0x[0-9A-F][0-9A-F] =>' \
    | sed 's/[[:space:]]*\(0x[0-9A-F][0-9A-F]\).*/"\1",/g')
CB_CODES="${CB_CODES%?}"
CB_CODES=$(echo $CB_CODES)

sed "s/{supportedCodes}/${SUPPORTED_CODES}/g" src/SupportedCodes.elm.template \
| sed "s/{supportedCBCodes}/${CB_CODES}/g" \
| elm-format --stdin \
> src/SupportedCodes.elm

git add docs/src/SupportedCodes.elm
git commit -m "Automated commit of supported codes"
