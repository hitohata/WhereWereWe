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

The ID is an auto-increment number.
Thus, there are ID manage tables.

* PK: Travel ID

* SK: Composite ID

#### TODO List Group

| Key        | Value                            | Type   |
|:-----------|:---------------------------------|:-------|
| PK         | Travel ID                        | String |
| SK         | ToDoListGroup#(todo list ID[^2]) | String | 
| TodoListId | The todo ID                      | Number | 
| Name       | todo list name                   | String |
| ShareWith  | users this todo list shares with | String |
| TZ         | timezone offset                  | number |

#### ToDo (Each)

| Key         | Value                                          | Type    |
|:------------|:-----------------------------------------------|:--------|
| PK          | Travel ID                                      | String  |
| SK          | ToDoList#(todo list ID[^2])#ToDo#(todo ID[^2]) | String  | 
| TodoId      | todo list group ID                             | Number  |
| Summary     | summary                                        | String  |
| Description | description. Can be null                       | String  |
| DueDate     | can be null. UTC. Timestamp                    | Number  | 
| Done        | done or not                                    | boolean | 


##### ToDo ID

##### ToDo List Group ID Counter

| Key   | Value             | Type   |
|:------|:------------------|:-------|
| PK    | Travel ID         | String |
| SK    | "ToDoListCounter" | String |
| Count | latest ID         | Number |

##### ToDo ID Counter

| Key   | Value                      | Type   |
|:------|:---------------------------|:-------|
| PK    | Travel ID                  | String |
| SK    | ToDoCounter#(todo list id) | String |
| Count | latest ID                  | Number |

[^1]: Just a Placeholder
[^2]: Variable. Actual ID


