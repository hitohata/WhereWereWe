target "project-doc" {
  cache-to = [
    "type=gha,ignore-error=true,mode=min,scope=project-doc,output=type=docker"
  ]
  cache-from = [
    "type=gha,scope=project-doc"
  ]
}
target "top-page-doc" {
  cache-to = [
    "type=gha,ignore-error=true,mode=min,scope=top-page-doc,output=type=docker"
  ]
  cache-from = [
    "type=gha,scope=top-page-doc"
  ]
}