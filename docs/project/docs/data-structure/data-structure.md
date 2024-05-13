# Data Structure

This system uses DynamoDB as a data store.

## Table Definition

The PK and SK is just set as "PK" as "SK".

### Global Secondary Index (GSI)

None

## User

The user's definition is the following.

* PK: User ID
* SK: "USER"

### User Attributes

| Key      | Value              | Type   |
|:---------|:-------------------|:-------|
| PK       | User id            | String |
| SK       | "USER"[^1]         | String | 
| Name     | username           | String |
| EMail    | e-mail address     | String |
| Partners | a list of User IDs | List   |


## Travel

### ToDo


* PK: Travel ID

* SK: Composite ID


#### ToDo (Each)


| Key         | Value                                          | Type    |
|:------------|:-----------------------------------------------|:--------|
| PK          | Travel ID                                      | String  |
| SK          | ToDoList#(todo list ID[^2])#ToDo#(todo ID[^2]) | String  | 
| Summary     | summary                                        | String  |
| Description | description. Can be null                       | String  |
| Due Date    | can be null                                    | String  | 
| Done        | done or not                                    | boolean | 

#### TODO List Group

| Key       | Value                            | Type   |
|:----------|:---------------------------------|:-------|
| PK        | Travel ID                        | String |
| SK        | ToDoList#(todo list ID[^2])      | String | 
| Name      | todo list name                   | String |
| ShareWith | users this todo list shares with | String |


[^1]: Just a Placeholder
[^2]: Variable. Actual ID


