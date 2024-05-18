"use strict";(self.webpackChunkproject=self.webpackChunkproject||[]).push([[819],{7355:(e,t,l)=>{l.r(t),l.d(t,{assets:()=>x,contentTitle:()=>i,default:()=>a,frontMatter:()=>r,metadata:()=>d,toc:()=>c});var n=l(4848),s=l(8453);const r={},i="Data Structure",d={id:"data-structure/data-structure",title:"Data Structure",description:"This system uses DynamoDB as a data store.",source:"@site/docs/data-structure/data-structure.md",sourceDirName:"data-structure",slug:"/data-structure/",permalink:"/WhereWereWe/project/data-structure/",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Schema of DB",permalink:"/WhereWereWe/project/data-structure/schema"}},x={},c=[{value:"Table Definition",id:"table-definition",level:2},{value:"Global Secondary Index (GSI)",id:"global-secondary-index-gsi",level:3},{value:"Local Secondary Index (LSI)",id:"local-secondary-index-lsi",level:3},{value:"User",id:"user",level:2},{value:"User Attributes",id:"user-attributes",level:3},{value:"Travel",id:"travel",level:2},{value:"Travel",id:"travel-1",level:3},{value:"Travel - User",id:"travel---user",level:3},{value:"ToDo",id:"todo",level:3},{value:"TODO List Group",id:"todo-list-group",level:4},{value:"ToDo (Each)",id:"todo-each",level:4},{value:"ToDo ID",id:"todo-id",level:5},{value:"ToDo List Group ID Counter",id:"todo-list-group-id-counter",level:5},{value:"ToDo ID Counter",id:"todo-id-counter",level:5}];function h(e){const t={a:"a",h1:"h1",h2:"h2",h3:"h3",h4:"h4",h5:"h5",li:"li",ol:"ol",p:"p",section:"section",sup:"sup",table:"table",tbody:"tbody",td:"td",th:"th",thead:"thead",tr:"tr",ul:"ul",...(0,s.R)(),...e.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(t.h1,{id:"data-structure",children:"Data Structure"}),"\n",(0,n.jsx)(t.p,{children:"This system uses DynamoDB as a data store."}),"\n",(0,n.jsx)(t.h2,{id:"table-definition",children:"Table Definition"}),"\n",(0,n.jsx)(t.p,{children:'The PK and SK is just set as "PK" as "SK".'}),"\n",(0,n.jsx)(t.h3,{id:"global-secondary-index-gsi",children:"Global Secondary Index (GSI)"}),"\n",(0,n.jsx)(t.p,{children:"None"}),"\n",(0,n.jsx)(t.h3,{id:"local-secondary-index-lsi",children:"Local Secondary Index (LSI)"}),"\n",(0,n.jsx)(t.p,{children:"Name: TravelDate"}),"\n",(0,n.jsxs)(t.ul,{children:["\n",(0,n.jsx)(t.li,{children:"PK: Travel ID"}),"\n",(0,n.jsx)(t.li,{children:"StartDate: start date of the travel"}),"\n"]}),"\n",(0,n.jsx)(t.h2,{id:"user",children:"User"}),"\n",(0,n.jsx)(t.p,{children:"The user's definition is the following."}),"\n",(0,n.jsxs)(t.ul,{children:["\n",(0,n.jsx)(t.li,{children:"PK: User ID"}),"\n",(0,n.jsx)(t.li,{children:'SK: "USER"'}),"\n"]}),"\n",(0,n.jsx)(t.h3,{id:"user-attributes",children:"User Attributes"}),"\n",(0,n.jsxs)(t.table,{children:[(0,n.jsx)(t.thead,{children:(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Key"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Value"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Type"})]})}),(0,n.jsxs)(t.tbody,{children:[(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"PK"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"User id"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"SK"}),(0,n.jsxs)(t.td,{style:{textAlign:"left"},children:['"USER"',(0,n.jsx)(t.sup,{children:(0,n.jsx)(t.a,{href:"#user-content-fn-1",id:"user-content-fnref-1","data-footnote-ref":!0,"aria-describedby":"footnote-label",children:"1"})})]}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Name"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"username"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"EMail"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"e-mail address"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Partners"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"a list of User IDs"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"List"})]})]})]}),"\n",(0,n.jsx)(t.h2,{id:"travel",children:"Travel"}),"\n",(0,n.jsx)(t.h3,{id:"travel-1",children:"Travel"}),"\n",(0,n.jsxs)(t.table,{children:[(0,n.jsx)(t.thead,{children:(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Key"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Value"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Type"})]})}),(0,n.jsxs)(t.tbody,{children:[(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"PK"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Travel id"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"SK"}),(0,n.jsxs)(t.td,{style:{textAlign:"left"},children:['"Travel"',(0,n.jsx)(t.sup,{children:(0,n.jsx)(t.a,{href:"#user-content-fn-1",id:"user-content-fnref-1-2","data-footnote-ref":!0,"aria-describedby":"footnote-label",children:"1"})})]}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Name"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"name of this travel"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"StartDate"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Start date (ISO8601)"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"EndDate"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"end date (ISO8601). Can be null"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Travelers"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"a list of User IDs"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"List"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"InvolvedUsers"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"a list of User IDs"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"List"})]})]})]}),"\n",(0,n.jsx)(t.h3,{id:"travel---user",children:"Travel - User"}),"\n",(0,n.jsx)(t.p,{children:"To bind the user and the travel."}),"\n",(0,n.jsxs)(t.table,{children:[(0,n.jsx)(t.thead,{children:(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Key"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Value"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Type"})]})}),(0,n.jsxs)(t.tbody,{children:[(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"PK"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"User ID"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"SK"}),(0,n.jsxs)(t.td,{style:{textAlign:"left"},children:["Travels#(Travel ID",(0,n.jsx)(t.sup,{children:(0,n.jsx)(t.a,{href:"#user-content-fn-2",id:"user-content-fnref-2","data-footnote-ref":!0,"aria-describedby":"footnote-label",children:"2"})}),")"]}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]})]})]}),"\n",(0,n.jsx)(t.h3,{id:"todo",children:"ToDo"}),"\n",(0,n.jsx)(t.p,{children:"The ID is an auto-increment number.\nThus, there are ID manage tables."}),"\n",(0,n.jsxs)(t.ul,{children:["\n",(0,n.jsxs)(t.li,{children:["\n",(0,n.jsx)(t.p,{children:"PK: Travel ID"}),"\n"]}),"\n",(0,n.jsxs)(t.li,{children:["\n",(0,n.jsx)(t.p,{children:"SK: Composite ID"}),"\n"]}),"\n"]}),"\n",(0,n.jsx)(t.h4,{id:"todo-list-group",children:"TODO List Group"}),"\n",(0,n.jsxs)(t.table,{children:[(0,n.jsx)(t.thead,{children:(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Key"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Value"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Type"})]})}),(0,n.jsxs)(t.tbody,{children:[(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"PK"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Travel ID"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"SK"}),(0,n.jsxs)(t.td,{style:{textAlign:"left"},children:["ToDoListGroup#(todo list ID",(0,n.jsx)(t.sup,{children:(0,n.jsx)(t.a,{href:"#user-content-fn-2",id:"user-content-fnref-2-2","data-footnote-ref":!0,"aria-describedby":"footnote-label",children:"2"})}),")"]}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"TodoListGroupId"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"The todo group ID"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Number"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Name"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"todo list name"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"TZ"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"timezone offset"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Number"})]})]})]}),"\n",(0,n.jsx)(t.h4,{id:"todo-each",children:"ToDo (Each)"}),"\n",(0,n.jsxs)(t.table,{children:[(0,n.jsx)(t.thead,{children:(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Key"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Value"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Type"})]})}),(0,n.jsxs)(t.tbody,{children:[(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"PK"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Travel ID"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"SK"}),(0,n.jsxs)(t.td,{style:{textAlign:"left"},children:["ToDoList#(todo list ID",(0,n.jsx)(t.sup,{children:(0,n.jsx)(t.a,{href:"#user-content-fn-2",id:"user-content-fnref-2-3","data-footnote-ref":!0,"aria-describedby":"footnote-label",children:"2"})}),")#ToDo#(todo ID",(0,n.jsx)(t.sup,{children:(0,n.jsx)(t.a,{href:"#user-content-fn-2",id:"user-content-fnref-2-4","data-footnote-ref":!0,"aria-describedby":"footnote-label",children:"2"})}),")"]}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"TodoId"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"todo list group ID"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Number"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Summary"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"summary"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Description"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"description. Can be null"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"DueDate"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"can be null. UTC. Timestamp"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Number"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Done"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"done or not"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"boolean"})]})]})]}),"\n",(0,n.jsx)(t.h5,{id:"todo-id",children:"ToDo ID"}),"\n",(0,n.jsx)(t.h5,{id:"todo-list-group-id-counter",children:"ToDo List Group ID Counter"}),"\n",(0,n.jsxs)(t.table,{children:[(0,n.jsx)(t.thead,{children:(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Key"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Value"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Type"})]})}),(0,n.jsxs)(t.tbody,{children:[(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"PK"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Travel ID"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"SK"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:'"ToDoListCounter"'}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Count"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"latest ID"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Number"})]})]})]}),"\n",(0,n.jsx)(t.h5,{id:"todo-id-counter",children:"ToDo ID Counter"}),"\n",(0,n.jsxs)(t.table,{children:[(0,n.jsx)(t.thead,{children:(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Key"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Value"}),(0,n.jsx)(t.th,{style:{textAlign:"left"},children:"Type"})]})}),(0,n.jsxs)(t.tbody,{children:[(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"PK"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Travel ID"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"SK"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"ToDoCounter#(todo list id)"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"String"})]}),(0,n.jsxs)(t.tr,{children:[(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Count"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"latest ID"}),(0,n.jsx)(t.td,{style:{textAlign:"left"},children:"Number"})]})]})]}),"\n",(0,n.jsxs)(t.section,{"data-footnotes":!0,className:"footnotes",children:[(0,n.jsx)(t.h2,{className:"sr-only",id:"footnote-label",children:"Footnotes"}),"\n",(0,n.jsxs)(t.ol,{children:["\n",(0,n.jsxs)(t.li,{id:"user-content-fn-1",children:["\n",(0,n.jsxs)(t.p,{children:["Just a Placeholder ",(0,n.jsx)(t.a,{href:"#user-content-fnref-1","data-footnote-backref":"","aria-label":"Back to reference 1",className:"data-footnote-backref",children:"\u21a9"})," ",(0,n.jsxs)(t.a,{href:"#user-content-fnref-1-2","data-footnote-backref":"","aria-label":"Back to reference 1-2",className:"data-footnote-backref",children:["\u21a9",(0,n.jsx)(t.sup,{children:"2"})]})]}),"\n"]}),"\n",(0,n.jsxs)(t.li,{id:"user-content-fn-2",children:["\n",(0,n.jsxs)(t.p,{children:["Variable. Actual ID ",(0,n.jsx)(t.a,{href:"#user-content-fnref-2","data-footnote-backref":"","aria-label":"Back to reference 2",className:"data-footnote-backref",children:"\u21a9"})," ",(0,n.jsxs)(t.a,{href:"#user-content-fnref-2-2","data-footnote-backref":"","aria-label":"Back to reference 2-2",className:"data-footnote-backref",children:["\u21a9",(0,n.jsx)(t.sup,{children:"2"})]})," ",(0,n.jsxs)(t.a,{href:"#user-content-fnref-2-3","data-footnote-backref":"","aria-label":"Back to reference 2-3",className:"data-footnote-backref",children:["\u21a9",(0,n.jsx)(t.sup,{children:"3"})]})," ",(0,n.jsxs)(t.a,{href:"#user-content-fnref-2-4","data-footnote-backref":"","aria-label":"Back to reference 2-4",className:"data-footnote-backref",children:["\u21a9",(0,n.jsx)(t.sup,{children:"4"})]})]}),"\n"]}),"\n"]}),"\n"]})]})}function a(e={}){const{wrapper:t}={...(0,s.R)(),...e.components};return t?(0,n.jsx)(t,{...e,children:(0,n.jsx)(h,{...e})}):h(e)}}}]);