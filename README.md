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
aws lambda update-function-configuration --function-name authorization-service-responder-Responder-CFGENERATEDID  --runtime provided
aws lambda update-function-code --function-name authorization-service-responder-Responder-CFGENERATEDID --zip-file fileb://lambda.zip
```

## Latency Analysis

Wondering why a lambda function that has 30ms invocation latency results in 450ms curl invocation?
Have curl tell you!

Output format is in `curl-format.txt`:

Run with something like `curl -s -w @curl-format.txt -H 'content-type: application/json' -d '{"query":"{__schema {types {name description fields {name description}}}}"}' https://yourstagehost.com/yourstagename/graphql -o /dev/null`

You'll get something like:
```
    time_namelookup:  0.005116
       time_connect:  0.095079
    time_appconnect:  0.298310 (<-- SSH handshake complete)
   time_pretransfer:  0.298432
      time_redirect:  0.000000
 time_starttransfer:  0.298473 (<-- first byte transferred back from server)
                    ----------
         time_total:  0.445382 (<-- where did 150ms go?)
```

It's even worse in us-west-1:
```
    time_namelookup:  0.005120
       time_connect:  0.095137
    time_appconnect:  0.298649
   time_pretransfer:  0.298819
      time_redirect:  0.000000
 time_starttransfer:  0.298884
                    ----------
         time_total:  0.880548
```

I don't yet know what's happening between `time_starttransfer` and `time_total`. It's a tiny
return (2676 bytes body + ~450 bytes headers = ~3KB). Hypothesis is "first byte" is returned
in advance of the lambda invocation and that the invocation time plus the APIGW-to-Lambda
machinery takes up the rest.
