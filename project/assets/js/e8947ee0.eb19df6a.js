"use strict";(self.webpackChunkproject=self.webpackChunkproject||[]).push([[852],{479:(e,n,r)=>{r.r(n),r.d(n,{assets:()=>l,contentTitle:()=>t,default:()=>c,frontMatter:()=>o,metadata:()=>a,toc:()=>i});var s=r(4848),d=r(8453);const o={},t="Models",a={id:"models/models",title:"Models",description:"User",source:"@site/docs/models/models.md",sourceDirName:"models",slug:"/models/",permalink:"/WhereWereWe/project/models/",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Where Were We",permalink:"/WhereWereWe/project/"},next:{title:"Users",permalink:"/WhereWereWe/project/models/Users/"}},l={},i=[{value:"User",id:"user",level:2},{value:"Travel",id:"travel",level:2}];function I(e){const n={h1:"h1",h2:"h2",mermaid:"mermaid",...(0,d.R)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(n.h1,{id:"models",children:"Models"}),"\n",(0,s.jsx)(n.h2,{id:"user",children:"User"}),"\n",(0,s.jsx)(n.mermaid,{value:"classDiagram\n    class UserId~ID~ {\n        String: id \n        generate() UserId\n    }\n    \n    class User~Entity~ { \n        UserId: id\n        String: name\n        String: email\n        List~UserId~: pertners\n        changeName(String) User\n    }\n    \n    UserId --* User"}),"\n",(0,s.jsx)(n.h2,{id:"travel",children:"Travel"}),"\n",(0,s.jsx)(n.mermaid,{value:"classDiagram\n    class TravelId~ID~ {\n        String: id\n    }\n    \n    class Travel~AggrigateRoot~ {\n        TravelId: id\n        String: name    \n        List~TravelerId~: travelers\n        List~InvolvedUserId~: involvedUsers\n        List~TodoGropId~: todos\n    }\n    \n    class TravelerGroup {\n        UserId~UserId~: travelers\n    }\n    \n    class UserId~ID~ {\n        String: id\n        geenerate() UserId\n    }\n    \n    class TravelerId~ID~ {\n    }\n    \n    class InvolvedUserId~ID~ {\n    }    \n    \n    class TodoListGroupId~ID~ {\n        String: id\n    }\n    \n    class TodoListGroup~AggrigateRoot~ {\n        TodoGroupId: id\n        List~UserId~: shareWith\n        List~Todo~: todoList\n        share(InvolvedUserId) Todo\n    }\n    \n    class TodoId~ID~ {\n        String: id\n        generate() TodoId\n    }\n    \n    class Todo~Entity~ {\n        TodoId: id\n        String: summary\n        String: descrition - nullable\n        String: dueDate\n        Boolean: done\n        new(): Todo\n        update(): Todo\n        toggleTodo(): Todo\n    }\n\n    UserId --|> TravelerId\n    UserId --|> InvolvedUserId\n    TravelId --* Travel\n    TravelerId --* Travel\n    InvolvedUserId --* Travel\n    Todo --* TodoListGroup\n    TodoListGroupId --* TodoListGroup\n    UserId --* TodoListGroup\n    TodoListGroupId --* Travel\n    TodoId --* Todo"})]})}function c(e={}){const{wrapper:n}={...(0,d.R)(),...e.components};return n?(0,s.jsx)(n,{...e,children:(0,s.jsx)(I,{...e})}):I(e)}}}]);