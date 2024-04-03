var searchIndex = new Map(JSON.parse('[\
["core_where_were_we",{"doc":"","t":"","n":[],"q":[],"d":[],"i":[],"f":"","c":[],"p":[],"b":[]}],\
["test_utils",{"doc":"","t":"CCCFNNONNHNNNNONNN","n":["infrastructure","db","dynamo_db_client","TestDynamoTable","borrow","borrow_mut","client","default","delete_table","dynamodb_test_client","from","generate_test_table","into","into_shared","table_name","try_from","try_into","type_id"],"q":[[0,"test_utils"],[1,"test_utils::infrastructure"],[2,"test_utils::infrastructure::db"],[3,"test_utils::infrastructure::db::dynamo_db_client"],[18,"aws_sdk_dynamodb::client"],[19,"core::result"],[20,"core::any"]],"d":["","","","The table struct for the test.","","","","","remove a table","The DynamoDB user client for the testl.","Returns the argument unchanged.","This function is used for the test.","Calls <code>U::from(self)</code>.","","","","",""],"i":[0,0,0,0,2,2,2,2,2,0,2,2,2,2,2,2,2,2],"f":"````{ce{}{}}0`{bd}{df}{{}h}{cc{}}244`{c{{j{e}}}{}{}}0{cl{}}","c":[],"p":[[1,"str"],[5,"TestDynamoTable",3],[1,"unit"],[5,"Client",18],[6,"Result",19],[5,"TypeId",20]],"b":[]}],\
["users",{"doc":"","t":"CCCCCCFNNNONNONNONNNNCPPPGNNNNNNNNNNNCCCCKMMFFNNNNNNNNNONNNNNNNNNNNNONNNNOONONNNNNNNNNNNFNNNNNNNNNNNNONNNNNNNCFNNONNNNNNNONNNCCFNNNNNNNNNNNOKMM","n":["dtos","errors","models","repository","use_case","user_dto","UserDto","borrow","borrow_mut","deserialize","email","from","from","id","into","into_shared","name","serialize","try_from","try_into","type_id","errors","InvalidUUID","UserNotFind","UsernameError","UsersError","borrow","borrow_mut","fmt","fmt","from","into","into_shared","to_string","try_from","try_into","type_id","repository","user","user_id","user_repository","UserRepository","find_by_id","save","User","Username","add_partner","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","email","eq","eq","equivalent","equivalent","equivalent","equivalent","equivalent","equivalent","fmt","fmt","from","from","id","into","into","into_shared","into_shared","name","name","new","partners","remove_partner","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","type_id","type_id","update_name","UserId","borrow","borrow_mut","clone","clone_into","eq","equivalent","equivalent","equivalent","fmt","from","generate","hash","id","into","into_shared","to_owned","try_from","try_from","try_into","type_id","user_repository","UserRepositoryConcrete","borrow","borrow_mut","client","default","find_by_id","fmt","from","into","into_shared","save","table_name","try_from","try_into","type_id","user_use_case_implementation","user_use_cases","CreateUserUseCaseInteractor","add_partner","borrow","borrow_mut","create","from","into","into_shared","new","try_from","try_into","type_id","user_repository","UserUseCases","add_partner","create"],"q":[[0,"users"],[5,"users::dtos"],[6,"users::dtos::user_dto"],[21,"users::errors"],[22,"users::errors::errors"],[37,"users::models"],[40,"users::models::repository"],[41,"users::models::repository::user_repository"],[44,"users::models::user"],[88,"users::models::user_id"],[109,"users::repository"],[110,"users::repository::user_repository"],[125,"users::use_case"],[127,"users::use_case::user_use_case_implementation"],[140,"users::use_case::user_use_cases"],[143,"core::result"],[144,"serde::de"],[145,"serde::ser"],[146,"core::any"],[147,"core::fmt"],[148,"core::fmt"],[149,"core::option"],[150,"anyhow"],[151,"alloc::vec"],[152,"core::hash"]],"d":["","","","","","","User DTO","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","user’s model","The User ID","","","","","","Username must be grater than 0 and less than equal 255.","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","User ID","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","Username","","","the partners if there is no partners, this value will be …","","","","","","","","","","","","User ID consists of an ID only that is UUID","","","","","","","","","","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","","","The argument is user ID that must be UUID. If you don’t …","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","create a new user","","","add a new partner","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","",""],"i":[0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,7,7,7,0,7,7,7,7,7,7,7,7,7,7,7,0,0,0,0,0,11,11,0,0,4,4,16,4,16,4,16,4,16,4,4,16,4,4,4,16,16,16,4,16,4,16,4,4,16,4,16,4,16,4,4,4,4,16,4,16,16,4,16,4,16,4,0,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,0,0,21,21,21,21,21,21,21,21,21,21,21,21,21,21,0,0,0,22,22,22,22,22,22,22,22,22,22,22,22,0,23,23],"f":"```````{ce{}{}}0{c{{d{b}}}f}`{hb}{cc{}}`33`{{bc}dj}{c{{d{e}}}{}{}}0{cl{}}`````66{{nA`}Ab}0477{cAd{}}332`````{{AfAh}{{Al{{Aj{h}}}}}}{{Afh}{{Al{An}}}}``{{hAh}An};;;;{hh}{B`B`}{{ce}An{}{}}0`{{hh}Bb}{{B`B`}Bb}{{ce}Bb{}{}}00000{{hA`}Ab}{{B`A`}Ab}{cc{}}0`{ce{}{}}000``{{AhB`Bd{Aj{{Bf{Ah}}}}}h}`;11{c{{d{e}}}{}{}}0{Bd{{d{B`n}}}}11{cl{}}0{{hB`}An}`55{AhAh}={{AhAh}Bb};;;{{AhA`}Ab}9{{}Ah}{{Ahc}AnBh}`:::{Bd{{d{Ahn}}}}997``;;`{{}Bj}{{BjAh}{{Al{{Aj{h}}}}}}{{BjA`}Ab}?>>{{Bjh}{{Al{An}}}}`==;```{{{Bl{c}}AhAh}{{d{bn}}}Af}{ce{}{}}0{{{Bl{c}}BdBd}{{d{bn}}}Af}{cc{}}22{c{{Bl{c}}}{}}{c{{d{e}}}{}{}}0{cl{}}``{{BnAhAh}{{d{bn}}}}{{BnBdBd}{{d{bn}}}}","c":[],"p":[[5,"UserDto",6],[6,"Result",143],[10,"Deserializer",144],[5,"User",44],[10,"Serializer",145],[5,"TypeId",146],[6,"UsersError",22],[5,"Formatter",147],[8,"Result",147],[5,"String",148],[10,"UserRepository",41],[5,"UserId",88],[6,"Option",149],[8,"Result",150],[1,"unit"],[5,"Username",44],[1,"bool"],[1,"str"],[5,"Vec",151],[10,"Hasher",152],[5,"UserRepositoryConcrete",110],[5,"CreateUserUseCaseInteractor",127],[10,"UserUseCases",140]],"b":[[28,"impl-Display-for-UsersError"],[29,"impl-Debug-for-UsersError"]]}],\
["utils",{"doc":"","t":"CCCCHCHH","n":["infrastructure","settings","db","dynamo_db_client","dynamodb_client","settings","dynamo_endpoint","table_name"],"q":[[0,"utils"],[2,"utils::infrastructure"],[3,"utils::infrastructure::db"],[4,"utils::infrastructure::db::dynamo_db_client"],[5,"utils::settings"],[6,"utils::settings::settings"],[8,"aws_sdk_dynamodb::client"]],"d":["","","","","","read environment values the following value are constant …","read a table name from the environment value This is used …","read a table name from the environment value"],"i":[0,0,0,0,0,0,0,0],"f":"````{{}b}`{{}d}0","c":[],"p":[[5,"Client",8],[1,"str"]],"b":[]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
