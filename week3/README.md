## Steps to run
- `make format` to format code
- `make lint` to lint
- `make release-arm` to build for arm which is: `cargo lambda build --release --arm64`
- `make deploy` which is this `cargo lambda deploy`

`pip3 install cargo-lambda` to install cargo-lambda

```rust
(.venv) @yifu-deng ➜ /workspaces/yifu_Weekly-miniproject/marco-polo-lambda (main) $ cargo lambda invoke --remote \
  --data-ascii '{"command": "hi"}' \
  --output-format json \
  rust-aws-lambda
{
  "body": "\"Hello from Lambda!\"",
  "statusCode": 200
}
```