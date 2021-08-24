local world_module = {}

function world_module.on_creation(world)
end

function world_module.on_reload(world)
end

function world_module.on_update(world, population)
    current_it = population[world.current_it]
    if current_it ~= nil and type(current_it.tag) == "table" and current_it.tag["It"] ~= NULL then
        world.recent_it = world.current_it
        world.current_it = current_it.tag["It"]
    end
end

return world_module
