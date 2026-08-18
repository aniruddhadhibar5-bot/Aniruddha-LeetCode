-export([randomized_set_init_/0, randomized_set_insert/1, randomized_set_remove/1, randomized_set_get_random/0]).

%% Initializes the RandomizedSet object.
randomized_set_init_() ->
    %% Create two in-memory ETS tables with unique names
    MapTable = ets:new(lc_map_table, [set, private]),
    ArrayTable = ets:new(lc_array_table, [set, private]),
    Size = 0,
    
    %% Store the components directly in the Process Dictionary
    put(map_table, MapTable),
    put(array_table, ArrayTable),
    put(set_size, Size),
    
    %% Return any arbitrary atom or reference since LeetCode's driver ignores the return value of init
    ok.

%% Inserts an item val into the set if not present.
randomized_set_insert(Val) ->
    MapTable = get(map_table),
    ArrayTable = get(array_table),
    Size = get(set_size),
    
    case ets:lookup(MapTable, Val) of
        [_Existing] ->
            false;
        [] ->
            ets:insert(MapTable, {Val, Size}),
            ets:insert(ArrayTable, {Size, Val}),
            put(set_size, Size + 1),
            true
    end.

%% Removes an item val from the set if present.
randomized_set_remove(Val) ->
    MapTable = get(map_table),
    ArrayTable = get(array_table),
    Size = get(set_size),
    
    case ets:lookup(MapTable, Val) of
        [] ->
            false;
        [{Val, IndexToRemove}] ->
            LastIndex = Size - 1,
            
            if 
                IndexToRemove =/= LastIndex ->
                    %% Fetch the element at the last position
                    [{LastIndex, LastVal}] = ets:lookup(ArrayTable, LastIndex),
                    %% Move the last element into the deleted element's position
                    ets:insert(ArrayTable, {IndexToRemove, LastVal}),
                    ets:insert(MapTable, {LastVal, IndexToRemove});
                true -> 
                    ok
            end,
            
            %% Delete the old values
            ets:delete(MapTable, Val),
            ets:delete(ArrayTable, LastIndex),
            
            put(set_size, LastIndex),
            true
    end.

%% Returns a random element from the current set of elements.
randomized_set_get_random() ->
    ArrayTable = get(array_table),
    Size = get(set_size),
    
    RandomIndex = rand:uniform(Size) - 1,
    [{RandomIndex, Val}] = ets:lookup(ArrayTable, RandomIndex),
    Val.
