-- Day/night cycle simulation
-- Attach to any entity with a SunData component (directional light)

function props()
    return {
        cycle_speed = { value = 1.0, hint = "How many in-game hours pass per real second" },
        start_hour = { value = 10.0, hint = "Starting hour (0-24)" },
        current_hour = { value = -1.0, hint = "Internal: current time" },
        last_logged_hour = { value = -1.0, hint = "Internal: last logged hour" },
    }
end

function on_ready()
    current_hour = start_hour or 10.0
    last_logged_hour = -1
    print_log("Day/night cycle started at hour " .. string.format("%.1f", current_hour))
end

function on_update()
    local speed = cycle_speed or 1.0
    current_hour = current_hour + speed * delta

    -- Wrap around 24 hours
    if current_hour >= 24.0 then
        current_hour = current_hour - 24.0
    end

    -- Log each whole hour change
    local hour_int = math.floor(current_hour)
    if hour_int ~= math.floor(last_logged_hour) then
        last_logged_hour = hour_int
        local label = "Night"
        if hour_int >= 6 and hour_int < 8 then label = "Dawn"
        elseif hour_int >= 8 and hour_int < 17 then label = "Day"
        elseif hour_int >= 17 and hour_int < 20 then label = "Dusk"
        end
        print_log(string.format("Time: %02d:00 — %s", hour_int, label))
    end

    -- Convert hour to sun elevation and azimuth
    local angle = (current_hour - 6.0) / 12.0 * 180.0
    local elevation = 90.0 * math.sin(math.rad(angle))
    local azimuth = (current_hour / 24.0) * 360.0

    -- Use generic reflection API to set sun angles
    set("sun.azimuth", azimuth)
    set("sun.elevation", elevation)
end
