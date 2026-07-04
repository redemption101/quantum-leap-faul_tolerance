-module(aetherdb).
-export([init/0, write_state/2, get_state/1]).

-record(q_state, {id, collapsed_value, timestamp}).

init() ->
    mnesia:create_schema([node()]),
    mnesia:start(),
    mnesia:create_table(q_state, [
        {attributes, record_info(fields, q_state)},
        {disc_copies, [node()]},
        {type, set}
    ]),
    io:format("AetherDB (Mnesia) Initialized. Ready for polyglot replication.~n").

write_state(Id, Value) ->
    F = fun() ->
        mnesia:write(#q_state{id = Id, collapsed_value = Value, timestamp = erlang:system_time()})
    end,
    mnesia:transaction(F).

get_state(Id) ->
    F = fun() -> mnesia:read({q_state, Id}) end,
    mnesia:transaction(F).
