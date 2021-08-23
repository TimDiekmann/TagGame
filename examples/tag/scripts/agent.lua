local agent_module = {}

math.randomseed(os.time())

function clamp(low, n, high)
    return math.min(math.max(n, low), high)
end
function tprint (t, s)
    for k, v in pairs(t) do
        local kfmt = '["' .. tostring(k) ..'"]'
        if type(k) ~= 'string' then
            kfmt = '[' .. k .. ']'
        end
        local vfmt = '"'.. tostring(v) ..'"'
        if type(v) == 'table' then
            tprint(v, (s or '')..kfmt)
        else
            if type(v) ~= 'string' then
                vfmt = tostring(v)
            end
            print(type(t)..(s or '')..kfmt..' = '..vfmt)
        end
    end
end

function run(state, board, dx, dy)
    state.position.x = clamp(0, state.position.x + dx, board.width - 1)
    state.position.y = clamp(0, state.position.y + dy, board.height - 1)
end

function distance_squared(p, q)
    return ((p.x - q.x)*(p.x - q.x))+((p.y - q.y)*(p.y - q.y))
end

function agent_module.on_creation(id, state, world)
end

function agent_module.on_reload(id, state, world)
end

function agent_module.on_update(id, state, world, population)
    if world.current_it == id then
        state.tag = 2
    elseif world.recent_id == id then
        state.tag = 1
    else
        state.tag = 0
    end

    if state.tag == 2 then
        nearest_d = 2000000000
        nearest_id = id

        for ag_id, agent in ipairs(population) do
            if id == ag_id then goto continue end
            if ag_id == world.recent_id then goto continue end

            d = distance_squared(population[world.current_it].position, state.position)
            if d < nearest_d then
                nearest_d = d
                nearest_id = ag_id
            end
            ::continue::
        end

        if nearest_id ~= id and nearest_d < 3 then
            -- population[world.recent_it].tag = 0
            -- population[world.current_it].tag = 1
            -- population[nearest_id].tag = 2
            world.recent_it = id
            world.current_it = nearest_id
        end

        if it_x < x then
            dx = 1
        else
            dx = -1
        end
        if math.random() > state.properties.tagged_deciding then
            dx = dx * -1
        end

        if it_y < y then
            dy = state.properties.tagged_deciding
        else
            dy = -state.properties.tagged_deciding
        end
        if math.random() > state.properties.tagged_deciding then
            dy = dy * -1
        end

        speed = state.properties.tagged_speed_multiplied
        run(state, world.board, dx*speed, dy*speed)

    elseif state.tag == 1 then

        if math.random() > 0.5 then
            dx = 1
        else
            dx = -1
        end
        if math.random() > 0.5 then
            dy = 1
        else
            dy = -1
        end
        run(state, world.board, dx*speed, dy*speed)

    elseif state.tag == 0 then
        pos_it = population[world.current_it].position
        it_x = pos_it.x
        it_y = pos_it.y
        x = state.position.x
        y = state.position.y

        if it_x < x then
            dx = 1
        else
            dx = -1
        end
        if math.random() > state.properties.untagged_deciding then
            dx = dx * -1
        end

        if it_y < y then
            dy = state.properties.untagged_deciding
        else
            dy = -state.properties.untagged_deciding
        end
        if math.random() > state.properties.untagged_deciding then
            dy = dy * -1
        end

        speed = state.properties.untagged_speed_multiplied
        run(state, world.board, dx*speed, dy*speed)
    end
end

return agent_module
