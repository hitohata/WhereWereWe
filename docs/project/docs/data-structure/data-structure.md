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

### TODO Group

| Key      | Value                  | Type   |
|:---------|:-----------------------|:-------|
| PK       | Travel ID              | String |
| SK       | TODOGroup#{number[^2]} | String | 
| Name     | username               | String |
| EMail    | e-mail address         | String |
| Partners | a list of User IDs     | List   |

### TODO

| Key      | Value                                     | Type   |
|:---------|:------------------------------------------|:-------|
| PK       | Travel ID                                 | String |
| SK       | TodoGroup#{number[^2]}#Todo#${number[^2]} | String | 
| Name     | username                                  | String |
| EMail    | e-mail address                            | String |
| Partners | a list of User IDs                        | List   |

[^1]: Just a Placeholder
[^2]: This is a variable
