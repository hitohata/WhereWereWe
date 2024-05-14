var searchIndex = new Map(JSON.parse('[\
["core_where_were_we",{"doc":"","t":"","n":[],"q":[],"d":[],"i":[],"f":"","c":[],"p":[],"b":[]}],\
["test_utils",{"doc":"","t":"CCCFNNONNHNNNNONNN","n":["infrastructure","db","dynamo_db_client","TestDynamoTable","borrow","borrow_mut","client","default","delete_table","dynamodb_test_client","from","generate_test_table","into","into_shared","table_name","try_from","try_into","type_id"],"q":[[0,"test_utils"],[1,"test_utils::infrastructure"],[2,"test_utils::infrastructure::db"],[3,"test_utils::infrastructure::db::dynamo_db_client"],[18,"aws_sdk_dynamodb::client"],[19,"core::result"],[20,"core::any"]],"d":["","","","The table struct for the test.","","","","","remove a table","The DynamoDB user client for the test.","Returns the argument unchanged.","This function is used for the test.","Calls <code>U::from(self)</code>.","","","","",""],"i":[0,0,0,0,2,2,2,2,2,0,2,2,2,2,2,2,2,2],"f":"````{ce{}{}}0`{bd}{df}{{}h}{cc{}}244`{c{{j{e}}}{}{}}0{cl{}}","c":[],"p":[[1,"str"],[5,"TestDynamoTable",3],[1,"unit"],[5,"Client",18],[6,"Result",19],[5,"TypeId",20]],"b":[]}],\
["travel",{"doc":"","t":"CCCCCPPGNNNNNNNNNNNCCCCKMMMMCCCCFNNNNOOONNONNNONNNNNNNHHFNNNNONNNNOOONNNCCFNNNNNNNNNNNNNONNNNNNFNNNNNNNNNNNNNONNNNNNCCCFNNNNNNNNNNNONONNNNNONONNNCCFNNNNNNNNNNNNNONNNNNNNFNNNNNNNNNNNONNNNNNNCFNNOHHNNNNNNNNNONNNCSSKFNNONNNNNMNMNNNONNN","n":["errors","models","repository","service","errors","DBError","DomainError","TravelError","borrow","borrow_mut","fmt","fmt","from","into","into_shared","to_string","try_from","try_into","type_id","repository","todo","travel","todo_repository","TodoRepository","find_todo_by_id","find_todo_group_by_id","save_todo","save_todo_group","entity","id","todo","todo_group","Todo","borrow","borrow_mut","clone","clone_into","description","done","due_date","fmt","from","id","into","into_shared","new","summary","to_owned","todo_id","toggle_todo","try_from","try_into","type_id","update","validate_description","validate_summary","TodoListGroup","add_todo","borrow","borrow_mut","from","group_name","into","into_shared","new","remove_todo","todo","todo_group_id","travel_id","try_from","try_into","type_id","todo_id","todo_list_group_id","TodoId","as_out","borrow","borrow_mut","clone","clone_into","eq","equivalent","equivalent","equivalent","fmt","from","from","id","id","into","into_shared","to_owned","try_from","try_into","type_id","TodoListGroupId","as_out","borrow","borrow_mut","clone","clone_into","eq","equivalent","equivalent","equivalent","fmt","from","from","id","id","into","into_shared","to_owned","try_from","try_into","type_id","entity","id","travel","Travel","add_traveler","borrow","borrow_mut","clone","clone_into","fmt","from","into","into_shared","involve_user","involved_users","involved_users","name","name","new","preclude_user","remove_traveler","to_owned","travel_id","travel_id","travelers","travelers","try_from","try_into","type_id","travel_id","user_id","TravelId","borrow","borrow_mut","clone","clone_into","eq","equivalent","equivalent","equivalent","fmt","from","generate","hash","id","id","into","into_shared","to_owned","try_from","try_from","try_into","type_id","UserId","borrow","borrow_mut","clone","clone_into","eq","equivalent","equivalent","equivalent","fmt","from","hash","id","into","into_shared","to_owned","try_from","try_from","try_into","type_id","todo_repository","TodoRepositoryConcrete","borrow","borrow_mut","client","convert_hashmap_into_option_string","convert_into_todo","default","find_todo_by_id","find_todo_group_by_id","fmt","from","into","into_shared","save_todo","save_todo_group","table_name","try_from","try_into","type_id","todo_id_service","TODO_KEY","TODO_LIST_GROUP_KEY","TodoIdService","TodoIdServiceConcrete","borrow","borrow_mut","client","count_up","default","fmt","from","get_count","get_todo_id","get_todo_id","get_todo_list_group_id","get_todo_list_group_id","into","into_shared","table_name","try_from","try_into","type_id"],"q":[[0,"travel"],[4,"travel::errors"],[5,"travel::errors::errors"],[19,"travel::models"],[22,"travel::models::repository"],[23,"travel::models::repository::todo_repository"],[28,"travel::models::todo"],[30,"travel::models::todo::entity"],[32,"travel::models::todo::entity::todo"],[56,"travel::models::todo::entity::todo_group"],[72,"travel::models::todo::id"],[74,"travel::models::todo::id::todo_id"],[95,"travel::models::todo::id::todo_list_group_id"],[116,"travel::models::travel"],[118,"travel::models::travel::entity"],[119,"travel::models::travel::entity::travel"],[145,"travel::models::travel::id"],[147,"travel::models::travel::id::travel_id"],[169,"travel::models::travel::id::user_id"],[189,"travel::repository"],[190,"travel::repository::todo_repository"],[209,"travel::service"],[210,"travel::service::todo_id_service"],[232,"core::fmt"],[233,"core::fmt"],[234,"core::result"],[235,"core::any"],[236,"core::option"],[237,"alloc::vec"],[238,"outref"],[239,"core::hash"],[240,"aws_sdk_dynamodb::types::_attribute_value"],[241,"std::collections::hash::map"]],"d":["","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","The repository trait of the todo","","","","","","","","to do struct","This is the collection of the to do list","","","","","","This value must be grater than 0 and less than equal 500.","This is false by default.","due date. this is time stamp","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","The done is false by default.","This value must be grater than 0 and less than equal 200.","","","","","","","update the summary and the description","","The summary length must be grater than 0 and less than 200.","","add a new todo to this collection","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","remove a todo from this collection","","","","","","","todo ID This is auto increment number","todo list group ID This is auto increment number","","","","","","","","","","","","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","","","","","","","","travel struct","","add a traveler into this travel","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","involve a user into this travel","","Same as the travelers, this is also HashSet","","the length must be grater than 0 and less than equal 255.","the travelers and the involved users can be None.","preclude an involved user from this travel","remove a traveler from this travel😢","","","","","Travelers are HashSet since the value cannot be …","","","","Travel ID","user’s ID This is reference only,","travel ID","","","","","","","","","","Returns the argument unchanged.","","","","","Calls <code>U::from(self)</code>.","","","","The argument is user ID that must be UUID. If you don’t …","","","","","","","","","","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","","","","","","This is implementation of the to do repository.","","","","","Convert the DynamoDB result hashmap into Option string","Convert the item (HashMap) into the To do struct","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","Getting id service Return a number that latest ID + 1","","","","","","","","update the counter saving the latest ID","","","Returns the argument unchanged.","This function gets the latest ID The return ID has already …","Get the latest + 1 to do list ID if there is no data in …","","Get the latest + 1 to do list ID if there is no data in …","","Calls <code>U::from(self)</code>.","","","","",""],"i":[0,0,0,0,0,1,1,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,7,7,7,7,0,0,0,0,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,0,0,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,0,0,0,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,0,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,0,0,0,0,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,0,0,0,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,0,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,0,0,27,27,27,0,0,27,27,27,27,27,27,27,27,27,27,27,27,27,0,0,0,0,0,28,28,28,28,28,28,28,28,29,28,29,28,28,28,28,28,28,28],"f":"````````{ce{}{}}0{{bd}f}0{cc{}}22{ch{}}{c{{j{e}}}{}{}}0{cl{}}`````{{nA`AbAd}{{j{{Ah{Af}}b}}}}{{nA`Ab}{{j{{Ah{Aj}}b}}}}{{nAf}{{j{Alb}}}}{{nAj}{{j{Alb}}}}`````99{AfAf}{{ce}Al{}{}}```{{Afd}f}:`<<{{AdAn{Ah{An}}{Ah{B`}}{Ah{Bb}}}{{j{Afb}}}}`={AfAd}{AfAl};;:{{AfAn{Ah{An}}}{{j{Alb}}}}{{{Ah{An}}}{{j{Alb}}}}{An{{j{Alb}}}}`{{AjAf}Al}{ce{}{}}0{cc{}}`11{{A`AbAn{Bd{Af}}}Aj}{{AjAd}Al}```{c{{j{e}}}{}{}}0{cl{}}```{c{{Bf{e}}}{}{}}66{AdAd}{{ce}Al{}{}}{{AdAd}Bb}{{ce}Bb{}{}}00{{Add}f}:{BhAd}{AdBh}`===998`7=={AbAb}6{{AbAb}Bb}555{{Abd}f}?{BhAb}{AbBh}`{ce{}{}}00??>````{{BjBl}Al}11{BjBj}={{Bjd}f}{cc{}}443{Bj{{Bd{Bl}}}}`{BjAn}`{{A`An{Bn{Bl}}{Ah{{Bn{Bl}}}}}{{j{Bjb}}}}6{{BjBl}{{j{Alb}}}}8{BjA`}`4`{c{{j{e}}}{}{}}0{cl{}}```;;{A`A`}{{ce}Al{}{}}{{A`A`}Bb}{{ce}Bb{}{}}00{{A`d}f}<{{}A`}{{A`c}AlC`}{A`An}`{ce{}{}}00:{An{{j{A`b}}}};:`11{BlBl}9{{BlBl}Bb}888{{Bld}f}{cc{}}{{Blc}AlC`}`666{c{{j{e}}}{}{}}{An{{j{Blb}}}}1{cl{}}``99`{{{Cd{hCb}}An}{{j{{Ah{h}}b}}}}{{{Cd{hCb}}}{{j{{Ah{Af}}b}}}}{{}Cf}{{CfA`AbAd}{{j{{Ah{Af}}b}}}}{{CfA`Ab}{{j{{Ah{Aj}}b}}}}{{Cfd}f}:??{{CfAf}{{j{Alb}}}}{{CfAj}{{j{Alb}}}}`::8`````{ce{}{}}0`{{ChA`AnBh}{{j{Alb}}}}{{}Ch}{{Chd}f}{cc{}}{{ChA`An}{{j{Bhb}}}}{{CjA`Ab}{{j{Bhb}}}}{{ChA`Ab}{{j{Bhb}}}}{{CjA`}{{j{Bhb}}}}{{ChA`}{{j{Bhb}}}}99`{c{{j{e}}}{}{}}0{cl{}}","c":[],"p":[[6,"TravelError",5],[5,"Formatter",232],[8,"Result",232],[5,"String",233],[6,"Result",234],[5,"TypeId",235],[10,"TodoRepository",23],[5,"TravelId",147],[5,"TodoListGroupId",95],[5,"TodoId",74],[5,"Todo",32],[6,"Option",236],[5,"TodoListGroup",56],[1,"unit"],[1,"str"],[1,"i64"],[1,"bool"],[5,"Vec",237],[5,"Out",238],[1,"u32"],[5,"Travel",119],[5,"UserId",169],[1,"slice"],[10,"Hasher",239],[6,"AttributeValue",240],[5,"HashMap",241],[5,"TodoRepositoryConcrete",190],[5,"TodoIdServiceConcrete",210],[10,"TodoIdService",210]],"b":[[10,"impl-Debug-for-TravelError"],[11,"impl-Display-for-TravelError"]]}],\
["users",{"doc":"","t":"CCCCCCFNNNONNONNNNNOONNNNNCPPPPPPGPNNNNNNNNNNNNNNNNNCCCCFFKOCCNNNNNNNNNNMNONNNNNNNNNNNNNNMNONNNNNNNNCCPFPFPPPGPPPPPGPPNNNNNNNNNNONNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNONNNNNNNNNNNNNONONNONNNNNNNNNNNNNNNNNNNNNNNNPFPFPPPGPPPPPGPPNNNNNNNNNNONNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNONNNNNNNNNNNNNONONNONNNNNNNNNNNNNNNNNNNNNNNNFFNNNNNNNNNONNNNNNNNNNNNONNNNNNNNNNOONONNNNNNNNNNNNNFNNNNNNNNNNNNNONNNNNNNNNNNCFNNONNNNNNNNNNONNNNCCFNNNNNNNNNNNNNHNNNNOKMMMM","n":["dtos","errors","models","repository","use_case","user_dto","UserDto","borrow","borrow_mut","deserialize","email","from","from","id","into","into_any","into_any_arc","into_any_rc","into_shared","name","partners","serialize","try_from","try_into","type_id","type_name","errors","Connection","DBError","DomainError","InvalidUUID","UserNotFind","UsernameError","UsersError","UsersUseCaseError","borrow","borrow_mut","fmt","fmt","from","from","into","into_any","into_any_arc","into_any_rc","into_shared","source","to_string","try_from","try_into","type_id","type_name","repository","user","user_id","user_repository","MockUserRepository","MockUserRepository_UserRepository","UserRepository","UserRepository_expectations","__mock_MockUserRepository","__mock_MockUserRepository_UserRepository","borrow","borrow","borrow_mut","borrow_mut","checkpoint","checkpoint","default","default","expect_find_by_id","expect_save","find_by_id","find_by_id","find_by_id","fmt","from","from","into","into","into_any","into_any","into_any_arc","into_any_arc","into_any_rc","into_any_rc","into_shared","into_shared","new","save","save","save","try_from","try_from","try_into","try_into","type_id","type_id","type_name","type_name","__find_by_id","__save","Always","Common","Default","Expectation","Expired","Func","FuncSt","Matcher","Mut","MutSt","Once","OnceSt","Pred","Rfunc","_Phantom","_Phantom","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","call","call_mut","common","default","default","default","default","drop","fmt","from","from","from","from","in_sequence","in_sequence","into","into","into","into","into_any","into_any","into_any","into_any","into_any_arc","into_any_arc","into_any_rc","into_any_rc","into_any_rc","into_any_rc","into_shared","into_shared","into_shared","into_shared","is_done","is_done","matcher","matches","matches","matches","never","never","new","once","return_const","return_const_st","return_once","return_once_st","returning","returning_st","rfunc","satisfy_sequence","seq_handle","times","times","times","to_string","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_name","type_name","type_name","type_name","verify_sequence","with","with","withf","withf","withf_st","withf_st","Always","Common","Default","Expectation","Expired","Func","FuncSt","Matcher","Mut","MutSt","Once","OnceSt","Pred","Rfunc","_Phantom","_Phantom","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","call","call_mut","common","default","default","default","default","drop","fmt","from","from","from","from","in_sequence","in_sequence","into","into","into","into","into_any","into_any","into_any","into_any","into_any_arc","into_any_arc","into_any_rc","into_any_rc","into_any_rc","into_any_rc","into_shared","into_shared","into_shared","into_shared","is_done","is_done","matcher","matches","matches","matches","never","never","new","once","return_const","return_const_st","return_once","return_once_st","returning","returning_st","rfunc","satisfy_sequence","seq_handle","times","times","times","to_string","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_name","type_name","type_name","type_name","verify_sequence","with","with","withf","withf","withf_st","withf_st","User","Username","add_partner","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","email","eq","eq","equivalent","equivalent","equivalent","equivalent","equivalent","equivalent","fmt","fmt","from","from","id","into","into","into_any","into_any","into_any_arc","into_any_arc","into_any_rc","into_any_rc","into_shared","into_shared","name","name","new","partners","remove_partner","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","type_id","type_id","type_name","type_name","update_name","UserId","borrow","borrow_mut","clone","clone_into","eq","equivalent","equivalent","equivalent","fmt","from","generate","hash","id","id","into","into_any","into_any_arc","into_any_rc","into_shared","to_owned","try_from","try_from","try_into","type_id","type_name","user_repository","UserRepositoryConcrete","borrow","borrow_mut","client","default","find_by_id","fmt","from","into","into_any","into_any_arc","into_any_rc","into_shared","save","table_name","try_from","try_into","type_id","type_name","user_use_case_implementation","user_use_cases","CreateUserUseCaseInteractor","add_partner","borrow","borrow_mut","change_name","create","from","into","into_any","into_any_arc","into_any_rc","into_shared","new","remove_partner","to_user_id_struct","try_from","try_into","type_id","type_name","user_repository","UserUseCases","add_partner","change_name","create","remove_partner"],"q":[[0,"users"],[5,"users::dtos"],[6,"users::dtos::user_dto"],[26,"users::errors"],[27,"users::errors::errors"],[52,"users::models"],[55,"users::models::repository"],[56,"users::models::repository::user_repository"],[100,"users::models::repository::user_repository::__mock_MockUserRepository_UserRepository"],[102,"users::models::repository::user_repository::__mock_MockUserRepository_UserRepository::__find_by_id"],[205,"users::models::repository::user_repository::__mock_MockUserRepository_UserRepository::__save"],[308,"users::models::user"],[360,"users::models::user_id"],[386,"users::repository"],[387,"users::repository::user_repository"],[406,"users::use_case"],[408,"users::use_case::user_use_case_implementation"],[428,"users::use_case::user_use_cases"],[433,"core::result"],[434,"serde::de"],[435,"alloc::boxed"],[436,"core::any"],[437,"alloc::sync"],[438,"alloc::rc"],[439,"serde::ser"],[440,"core::any"],[441,"core::fmt"],[442,"core::error"],[443,"core::option"],[444,"alloc::string"],[445,"core::fmt"],[446,"core::clone"],[447,"core::convert"],[448,"core::marker"],[449,"core::ops::function"],[450,"core::ops::function"],[451,"core::ops::function"],[452,"core::hash"]],"d":["","","","","","","User DTO","","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","user’s model","The User ID","","","","","","","","","","","","Validate that all current expectations for all methods have","Validate that all current expectations for all methods have","","","Create an <code>Expectation</code> for mocking the <code>find_by_id</code> method","Create an <code>Expectation</code> for mocking the <code>save</code> method","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","Create a new mock object with no expectations.","","","","","","","","","","","","","","","Holds the stuff that is independent of the output type","","Expectation type for methods that return a <code>&#39;static</code> type. …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","Add this expectation to a <code>Sequence</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","Validate this expectation’s matcher.","Forbid this expectation from ever being called.","Forbid this expectation from ever being called.","Create a new, default, <code>Expectation</code>","Expect this expectation to be called exactly once.  …","Return a constant value from the <code>Expectation</code>","Single-threaded version of <code>return_const</code>.  This is useful …","Supply an <code>FnOnce</code> closure that will provide the return …","Single-threaded version of <code>return_once</code>.  This is useful for","Supply a closure that will provide the return value for …","Single-threaded version of <code>returning</code>. Can be used when the …","","","","Expect this expectation to be called any number of times …","Restrict the number of times that that this method may be …","","","","","","","","","","","","","","","","","","","","","Set matching criteria for this Expectation.","","Set a matching function for this Expectation.","","Single-threaded version of <code>withf</code>. Can be used when the …","","Holds the stuff that is independent of the output type","","Expectation type for methods that return a <code>&#39;static</code> type. …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","Add this expectation to a <code>Sequence</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","Validate this expectation’s matcher.","Forbid this expectation from ever being called.","Forbid this expectation from ever being called.","Create a new, default, <code>Expectation</code>","Expect this expectation to be called exactly once.  …","Return a constant value from the <code>Expectation</code>","Single-threaded version of <code>return_const</code>.  This is useful …","Supply an <code>FnOnce</code> closure that will provide the return …","Single-threaded version of <code>return_once</code>.  This is useful for","Supply a closure that will provide the return value for …","Single-threaded version of <code>returning</code>. Can be used when the …","","","","Expect this expectation to be called any number of times …","Restrict the number of times that that this method may be …","","","","","","","","","","","","","","","","","","","","","Set matching criteria for this Expectation.","","Set a matching function for this Expectation.","","Single-threaded version of <code>withf</code>. Can be used when the …","","Username must be grater than 0 and less than equal 255.","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","User ID","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","Username","","","the partners if there is no partners, this value will be …","","","","","","","","","","","","","","User ID consists of an ID only that is UUID","","","","","","","","","","Returns the argument unchanged.","","","","","Calls <code>U::from(self)</code>.","","","","","","The argument is user ID that must be UUID. If you don’t …","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","","create a new user","","","Add a new partner The partner is added to the user. The …","","","update a user’s name","create a new user","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","Remove a new partner The partner is removed from the user. …","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,12,12,12,12,12,12,0,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,0,0,0,0,0,0,0,19,0,0,19,21,19,21,19,21,19,21,19,19,24,19,21,19,19,21,19,21,19,21,19,21,19,21,19,21,19,24,19,21,19,21,19,21,19,21,19,21,0,0,29,0,28,0,28,29,29,0,28,28,28,28,29,0,28,29,28,29,27,22,28,29,27,22,27,28,22,28,29,27,22,27,29,28,29,27,22,27,22,28,29,27,22,28,29,27,22,27,22,28,29,27,22,28,29,27,22,27,22,27,29,27,22,27,22,22,22,22,22,22,22,22,22,22,27,27,27,22,27,29,28,29,27,22,28,29,27,22,28,29,27,22,28,29,27,22,27,27,22,27,22,27,22,43,0,42,0,42,43,43,0,42,42,42,42,43,0,42,43,42,43,41,23,42,43,41,23,41,42,23,42,43,41,23,41,43,42,43,41,23,41,23,42,43,41,23,42,43,41,23,41,23,42,43,41,23,42,43,41,23,41,23,41,43,41,23,41,23,23,23,23,23,23,23,23,23,23,41,41,41,23,41,43,42,43,41,23,42,43,41,23,42,43,41,23,42,43,41,23,41,41,23,41,23,41,23,0,0,4,4,44,4,44,4,44,4,44,4,4,44,4,4,4,44,44,44,4,44,4,44,4,4,44,4,44,4,44,4,44,4,44,4,44,4,4,4,4,44,4,44,44,4,44,4,44,4,44,4,0,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,0,0,47,47,47,47,47,47,47,47,47,47,47,47,47,47,47,47,47,47,0,0,0,48,48,48,48,48,48,48,48,48,48,48,48,48,0,48,48,48,48,48,0,50,50,50,50],"f":"```````{ce{}{}}0{c{{d{b}}}f}`{cc{}}{hb}`3{{{j{c}}}{{j{l}}}{}}{{{n{c}}}{{n{l}}}{}}{{{A`{c}}}{{A`{l}}}{}}6``{{bc}dAb}{c{{d{e}}}{}{}}0{cAd{}}{cAf{}}`````````::{{AhAj}Al}0{AnAh}:<876<{Ah{{Bb{B`}}}}{cBd{}}6654``````````>>>>{BfBh}{BjBh}{{}Bf}{{}Bj}{BfBl}{BfBn}{{C`Cb}{{d{{Bb{h}}Ah}}}}{{BfCb}{{d{{Bb{h}}Ah}}}}`{{BfAj}{{d{BhCd}}}}{cc{}}0{ce{}{}}0{{{j{c}}}{{j{l}}}{}}0{{{n{c}}}{{n{l}}}{}}0{{{A`{c}}}{{A`{l}}}{}}033;{{C`h}{{d{BhAh}}}}{{Bfh}{{d{BhAh}}}}`{c{{d{e}}}{}{}}000{cAd{}}0{cAf{}}0``````````````````88888888{{CfAf}Bh}{{ChCb}{{d{{d{{Bb{h}}Ah}}Af}}}}`{{}Ch}{{}Cj}{{}Cf}{{}Bl}{CfBh}{{CjAj}Al}{cc{}}000{{CfCl}Cf}{{BlCl}Bl}{ce{}{}}000{{{j{c}}}{{j{l}}}{}}000{{{n{c}}}{{n{l}}}{}}0{{{A`{c}}}{{A`{l}}}{}}0003333{CfCn}{BlCn}`{{CjCb}Cn}{{CfCb}Cn}{{BlCb}Cn}={BlBl}?0{{Blc}Bl{D`{Db{{d{{Bb{h}}Ah}}}}Dd}}{{Blc}Bl{D`{Db{{d{{Bb{h}}Ah}}}}}}{{Blc}Bl{{Dh{Cb}{{Df{{d{{Bb{h}}Ah}}}}}}Dd}}{{Blc}Bl{{Dh{Cb}{{Df{{d{{Bb{h}}Ah}}}}}}}}{{Blc}Bl{{Dj{Cb}{{Df{{d{{Bb{h}}Ah}}}}}}Dd}}{{Blc}Bl{{Dj{Cb}{{Df{{d{{Bb{h}}Ah}}}}}}}}`{CfBh}`{{Cfc}Bh{{Db{Dl}}}}{{Blc}Bl{{Db{Dl}}}}`{cBd{}}{c{{d{e}}}{}{}}0000000{cAd{}}000{cAf{}}000{{CfAf}Bh}{{Cfc}Bh{{Dn{Cb}}Dd}}{{Blc}Bl{{Dn{Cb}}Dd}}{{Cfc}Bh{{E`{Cb}{{Df{Cn}}}}Dd}}{{Blc}Bl{{E`{Cb}{{Df{Cn}}}}Dd}}{{Cfc}Bh{{E`{Cb}{{Df{Cn}}}}}}{{Blc}Bl{{E`{Cb}{{Df{Cn}}}}}}````````````````{ce{}{}}0000000{{EbAf}Bh}{{Edh}{{d{{d{BhAh}}Af}}}}`{{}Ed}{{}Ef}{{}Eb}{{}Bn}{EbBh}{{EfAj}Al}{cc{}}000{{EbCl}Eb}{{BnCl}Bn};;;;{{{j{c}}}{{j{l}}}{}}000{{{n{c}}}{{n{l}}}{}}0{{{A`{c}}}{{A`{l}}}{}}000>>>>{EbCn}{BnCn}`{{Efh}Cn}{{Ebh}Cn}{{Bnh}Cn}<{BnBn}>0{{Bnc}Bn{D`{Db{{d{BhAh}}}}Dd}}{{Bnc}Bn{D`{Db{{d{BhAh}}}}}}{{Bnc}Bn{{Dh{h}{{Df{{d{BhAh}}}}}}Dd}}{{Bnc}Bn{{Dh{h}{{Df{{d{BhAh}}}}}}}}{{Bnc}Bn{{Dj{h}{{Df{{d{BhAh}}}}}}Dd}}{{Bnc}Bn{{Dj{h}{{Df{{d{BhAh}}}}}}}}`{EbBh}`{{Ebc}Bh{{Db{Dl}}}}{{Bnc}Bn{{Db{Dl}}}}`{cBd{}}{c{{d{e}}}{}{}}0000000{cAd{}}000{cAf{}}000{{EbAf}Bh}{{Ebc}Bh{{Dn{h}}Dd}}{{Bnc}Bn{{Dn{h}}Dd}}{{Ebc}Bh{{E`{h}{{Df{Cn}}}}Dd}}{{Bnc}Bn{{E`{h}{{Df{Cn}}}}Dd}}{{Ebc}Bh{{E`{h}{{Df{Cn}}}}}}{{Bnc}Bn{{E`{h}{{Df{Cn}}}}}}``{{hCb}Bh}{ce{}{}}000{hh}{EhEh}{{ce}Bh{}{}}0`{{hh}Cn}{{EhEh}Cn}{{ce}Cn{}{}}00000{{hAj}Al}{{EhAj}Al}{cc{}}0`99{{{j{c}}}{{j{l}}}{}}0{{{n{c}}}{{n{l}}}{}}0{{{A`{c}}}{{A`{l}}}{}}0<<``{{CbEhAf{Bb{{Ej{Cb}}}}}h}`>=={c{{d{e}}}{}{}}{Af{{d{EhAh}}}}111{cAd{}}0{cAf{}}0{{hEh}Bh}`{ce{}{}}0{CbCb}{{ce}Bh{}{}}{{CbCb}Cn}{{ce}Cn{}{}}00{{CbAj}Al}?{{}Cb}{{Cbc}BhEl}{CbAf}`8{{{j{c}}}{{j{l}}}{}}{{{n{c}}}{{n{l}}}{}}{{{A`{c}}}{{A`{l}}}{}};;{Af{{d{CbAh}}}}{c{{d{e}}}{}{}}0{cAd{}}{cAf{}}``??`{{}En}{{EnCb}{{d{{Bb{h}}Ah}}}}{{EnAj}Al}{cc{}}{ce{}{}};:90{{Enh}{{d{BhAh}}}}`8876```{{{F`{c}}AfAf}{{d{bAh}}}C`}220032=<;2{c{{F`{c}}}{}}1{{AfAf}{{d{{Fb{CbCb}}Ah}}}};;:9``{{FdAfAf}{{d{bAh}}}}000","c":[],"p":[[5,"UserDto",6],[6,"Result",433],[10,"Deserializer",434],[5,"User",308],[5,"Box",435],[10,"Any",436],[5,"Arc",437],[5,"Rc",438],[10,"Serializer",439],[5,"TypeId",436],[1,"str"],[6,"UsersError",27],[5,"Formatter",440],[8,"Result",440],[6,"Error",441],[10,"Error",442],[6,"Option",443],[5,"String",444],[5,"MockUserRepository",56],[1,"unit"],[5,"MockUserRepository_UserRepository",56],[5,"Expectation",102],[5,"Expectation",205],[10,"UserRepository",56],[5,"UserId",360],[5,"Error",440],[5,"Common",102],[6,"Rfunc",102],[6,"Matcher",102],[5,"Sequence",445],[1,"bool"],[10,"Clone",446],[10,"Into",447],[10,"Send",448],[17,"Output"],[10,"FnOnce",449],[10,"FnMut",449],[5,"TimesRange",445],[10,"Predicate",450],[10,"Fn",449],[5,"Common",205],[6,"Rfunc",205],[6,"Matcher",205],[5,"Username",308],[5,"Vec",451],[10,"Hasher",452],[5,"UserRepositoryConcrete",387],[5,"CreateUserUseCaseInteractor",408],[1,"tuple"],[10,"UserUseCases",428]],"b":[[37,"impl-Debug-for-UsersError"],[38,"impl-Display-for-UsersError"]]}],\
["utils",{"doc":"","t":"CCCCHCHH","n":["infrastructure","settings","db","dynamo_db_client","dynamodb_client","settings","dynamo_endpoint","table_name"],"q":[[0,"utils"],[2,"utils::infrastructure"],[3,"utils::infrastructure::db"],[4,"utils::infrastructure::db::dynamo_db_client"],[5,"utils::settings"],[6,"utils::settings::settings"],[8,"aws_sdk_dynamodb::client"]],"d":["","","","","","read environment values the following value are constant …","read a table name from the environment value This is used …","read a table name from the environment value"],"i":[0,0,0,0,0,0,0,0],"f":"````{{}b}`{{}d}0","c":[],"p":[[5,"Client",8],[1,"str"]],"b":[]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
