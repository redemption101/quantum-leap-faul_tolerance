-module(aetherion_app).
-behaviour(application).

-export([start/2, stop/1]).

start(_StartType, _StartArgs) ->
    %% Initialize AetherDB (Mnesia) upon application start
    aetherdb:init(),
    aetherion_sup:start_link().

stop(_State) ->
    ok.
