target "www-core-dynamo" {
  cache-to = [
    "type=gha,ignore-error=true,mode=max,scope=www-core-dynamo"
  ]
  cache-from = [
    "type=gha,scope=www-core-dynamo"
  ]
}