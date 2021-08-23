#![allow(warnings)]

pub trait ScriptHost<S: Serialize, W: Serialize> {
    type ScriptContext;
    type Error;
    fn create_context(world: &W) -> Result<Self::ScriptContext, Self::Error>;

    fn add_agent_behavior(
        context: &mut Self::ScriptContext,
        source: &str,
    ) -> Result<u32, Self::Error>;
    fn add_world_behavior(
        context: &mut Self::ScriptContext,
        source: &str,
    ) -> Result<u32, Self::Error>;

    fn update_agent_behavior(
        context: &mut Self::ScriptContext,
        id: u32,
        source: &str,
    ) -> Result<(), Self::Error>;

    fn update_world_behavior(
        context: &mut Self::ScriptContext,
        id: u32,
        source: &str,
    ) -> Result<(), Self::Error>;

    fn add_agent(
        context: &mut Self::ScriptContext,
        behavior: u32,
        state: S,
    ) -> Result<usize, Self::Error>;

    fn invoke(context: &mut Self::ScriptContext) -> Result<(), Self::Error>;

    fn world<'de>(context: &'de Self::ScriptContext) -> Result<W, Self::Error>
    where
        W: Deserialize<'de>;
    fn agents<'de>(context: &'de Self::ScriptContext) -> Result<Vec<S>, Self::Error>
    where
        S: Deserialize<'de>;
}

use mlua::{Error, Function, Lua, LuaSerdeExt, Table, Value};
use serde::{Deserialize, Serialize};

pub struct LuaScriptHost;

impl LuaScriptHost {
    fn check_behavior(context: &mut Lua, id: u32) -> Result<(), Error> {
        let behaviors: Table = context.globals().get("behaviors")?;
        if behaviors.contains_key(id)? {
            Ok(())
        } else {
            Err(Error::RuntimeError(format!(
                "Behavior `{}` does not exist",
                id
            )))
        }
    }
}

pub struct LuaScriptContext {
    lua: Lua,
    next_agent_id: usize,
}

impl<S: Serialize, W: Serialize> ScriptHost<S, W> for LuaScriptHost {
    type ScriptContext = LuaScriptContext;
    type Error = Error;

    fn create_context(world: &W) -> Result<LuaScriptContext, Error> {
        let lua = Lua::new();

        lua.globals().set("world", lua.to_value(&world)?)?;

        let world_behaviors = lua.create_table()?;
        lua.globals().set("world_behaviors", world_behaviors)?;

        let behaviors = lua.create_table()?;
        lua.globals().set("behaviors", behaviors)?;

        let agents = lua.create_table()?;
        lua.globals().set("agent_behaviors", agents)?;

        let states = lua.create_table()?;
        lua.globals().set("states", states)?;

        Ok(LuaScriptContext {
            lua,
            next_agent_id: 1,
        })
    }

    fn add_agent_behavior(
        context: &mut Self::ScriptContext,
        source: &str,
    ) -> Result<u32, Self::Error> {
        let behaviors: Table = context.lua.globals().get("behaviors")?;
        let id = behaviors.len()? as u32 + 1;
        let behavior: Table = context.lua.load(source).eval()?;
        behaviors.set(id, behavior)?;
        Ok(id)
    }

    fn add_world_behavior(
        context: &mut Self::ScriptContext,
        source: &str,
    ) -> Result<u32, Self::Error> {
        let behaviors: Table = context.lua.globals().get("world_behaviors")?;
        let id = behaviors.len()? as u32 + 1;
        let behavior: Table = context.lua.load(source).eval()?;
        if behavior.contains_key("on_creation")? {
            let on_creation: Function = behavior.get("on_creation")?;
            let world: Table = context.lua.globals().get("world")?;
            on_creation.call::<_, ()>(world)?;
        }
        behaviors.set(id, behavior)?;
        Ok(id)
    }

    fn update_agent_behavior(
        context: &mut Self::ScriptContext,
        behavior_id: u32,
        source: &str,
    ) -> Result<(), Self::Error> {
        let behaviors: Table = context.lua.globals().get("behaviors")?;
        if !behaviors.contains_key(behavior_id)? {
            return Err(Error::RuntimeError(format!(
                "Agent behavior `{}` does not exist",
                behavior_id
            )));
        }

        // Update behavior
        let behavior: Table = context.lua.load(source).eval()?;
        let on_reload: Option<Function> = behavior
            .contains_key("on_reload")?
            .then(|| behavior.get("on_reload"))
            .transpose()?;
        behaviors.set(behavior_id, behavior)?;

        // Call `on_reload`
        if let Some(on_reload) = on_reload {
            let states: Table = context.lua.globals().get("states")?;
            let world: Table = context.lua.globals().get("world")?;
            let agent_behavior: Table = context.lua.globals().get("agent_behaviors")?;
            for v in agent_behavior.pairs() {
                let (id, ag_behavior_id): (usize, u32) = v?;
                if ag_behavior_id == behavior_id {
                    let state: Table = states.get(id)?;
                    on_reload.call::<_, ()>((id, state, world.clone()))?;
                }
            }
        }
        Ok(())
    }

    fn update_world_behavior(
        context: &mut Self::ScriptContext,
        behavior_id: u32,
        source: &str,
    ) -> Result<(), Self::Error> {
        let behaviors: Table = context.lua.globals().get("world_behaviors")?;
        if !behaviors.contains_key(behavior_id)? {
            return Err(Error::RuntimeError(format!(
                "World behavior `{}` does not exist",
                behavior_id
            )));
        }

        // Update behavior
        let behavior: Table = context.lua.load(source).eval()?;
        let on_reload: Option<Function> = behavior
            .contains_key("on_reload")?
            .then(|| behavior.get("on_reload"))
            .transpose()?;
        behaviors.set(behavior_id, behavior)?;

        // Call `on_reload`
        if let Some(on_reload) = on_reload {
            let world: Table = context.lua.globals().get("world")?;
            on_reload.call::<_, ()>(world)?;
        }
        Ok(())
    }

    fn add_agent(
        context: &mut LuaScriptContext,
        behavior_id: u32,
        state: S,
    ) -> Result<usize, Error> {
        Self::check_behavior(&mut context.lua, behavior_id)?;

        let id = context.next_agent_id;
        context.next_agent_id += 1;

        // assign behavior
        let agent_behaviors: Table = context.lua.globals().get("agent_behaviors")?;
        agent_behaviors.set(id, behavior_id)?;

        // assign state
        let states: Table = context.lua.globals().get("states")?;
        states.set(id, context.lua.to_value(&state)?)?;

        // call `on_creation`
        let behaviors: Table = context.lua.globals().get("behaviors")?;
        let behavior: Table = behaviors.get(behavior_id)?;
        if behavior.contains_key("on_creation")? {
            let world: Table = context.lua.globals().get("world")?;
            let on_creation: Function = behavior.get("on_creation")?;
            on_creation.call::<_, ()>((id, context.lua.to_value(&state)?, world))?;
        }
        Ok(id)
    }

    fn invoke(context: &mut LuaScriptContext) -> Result<(), Self::Error> {
        // collect states
        let states = context
            .lua
            .globals()
            .get::<_, Table>("states")?
            .sequence_values()
            .collect::<Result<Vec<Table>, Error>>()?;

        // collect states
        let behaviors = context
            .lua
            .globals()
            .get::<_, Table>("behaviors")?
            .sequence_values()
            .collect::<Result<Vec<Table>, Error>>()?;
        let on_update = behaviors
            .into_iter()
            .map(|table| table.get("on_update"))
            .collect::<Result<Vec<Function>, Error>>()?;

        // collect states
        let agent_behaviors = context
            .lua
            .globals()
            .get::<_, Table>("agent_behaviors")?
            .sequence_values()
            .collect::<Result<Vec<usize>, Error>>()?;

        let all_states = context.lua.to_value(&states)?;
        let world: Table = context.lua.globals().get("world")?;

        states
            .into_iter()
            .enumerate()
            .map(|(id, state)| {
                on_update[agent_behaviors[id] - 1].call::<_, ()>((
                    id + 1,
                    state,
                    world.clone(),
                    all_states.clone(),
                ))
            })
            .collect::<Result<Vec<()>, Error>>()?;

        // collect states
        let states: Table = context.lua.globals().get("states")?;
        let world_behaviors: Table = context.lua.globals().get("world_behaviors")?;
        for world_behavior in world_behaviors.sequence_values() {
            let world_behavior: Table = world_behavior?;
            if world_behavior.contains_key("on_update")? {
                let on_update: Function = world_behavior.get("on_update")?;
                on_update.call::<_, ()>((world.clone(), states.clone()))?;
            }
        }

        Ok(())
    }

    fn world<'de>(context: &'de Self::ScriptContext) -> Result<W, Self::Error>
    where
        W: Deserialize<'de>,
    {
        context
            .lua
            .from_value(Value::Table(context.lua.globals().get("world")?))
    }
    fn agents<'de>(context: &'de Self::ScriptContext) -> Result<Vec<S>, Self::Error>
    where
        S: Deserialize<'de>,
    {
        // collect states
        let states = context
            .lua
            .globals()
            .get::<_, Table>("states")?
            .sequence_values()
            .collect::<Result<Vec<Table>, Error>>()?;

        states
            .into_iter()
            .map(|state| context.lua.from_value(Value::Table(state)))
            .collect()
    }
}
