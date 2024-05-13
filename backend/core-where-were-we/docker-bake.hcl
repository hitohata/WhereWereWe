target "www-core-app" {
  cache-to = [
    "type=gha,ignore-error=true,mode=min,scope=www-core-app"
  ]
  cache-from = [
    "type=gha,scope=www-core-app"
  ]
}
target "www-core-dynamo-db" {
  cache-to = [
    "type=gha,ignore-error=true,mode=min,scope=www-core-dynamo-db"
  ]
  cache-from = [
    "type=gha,scope=www-core-dynamo-db"
  ]
}