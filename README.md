# Authorization Service

You know, authorize things.

## Local Build For Lambda

```sh
TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/authorization_service ./bootstrap
zip lambda bootstrap
```

## Update Lambda

```sh
aws lambda update-function-configuration --function-name authorization-service-responder-Responder-CV7LGR6IIVFM  --runtime provided
aws lambda update-function-code --function-name authorization-service-responder-Responder-CV7LGR6IIVFM --zip-file fileb://lambda.zip
```

