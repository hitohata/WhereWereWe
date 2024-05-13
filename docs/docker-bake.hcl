target "project-doc" {
  context = "./project"
  cache-to = [
    "type=gha,ignore-error=true,mode=max,scope=project-doc"
  ]
  cache-from = [
    "type=gha,scope=project-doc"
  ]
}
target "top-page-doc" {
  cache-to = [
    "type=gha,ignore-error=true,mode=max,scope=top-page-doc"
  ]
  cache-from = [
    "type=gha,scope=top-page-doc"
  ]
}