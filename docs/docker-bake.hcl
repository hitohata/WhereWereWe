
target "top-page-doc" {
  default = []
  cache-to = [
    "type=gha,ignore-error=true,mode=max,scope=top-page-doc"
  ]
  cache-from = [
    "type=gha,scope=top-page-doc"
  ]
}