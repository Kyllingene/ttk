use mlua::{Lua, Result, Table};

macro_rules! table {
    ($lua:ident, $exports:ident; $($fn:ident = $callback:expr),* $(,)?) => {
        $( $exports.set(stringify!($fn), $lua.create_function($callback)?)?; )*
    };
}

#[mlua::lua_module]
fn ttk(lua: &Lua) -> Result<Table> {
    let exports = lua.create_table()?;
    exports.set("cmd", cmd(lua)?)?;
    exports.set("display", display(lua)?)?;
    exports.set("exit", exit(lua)?)?;
    exports.set("logfile", logfile(lua)?)?;
    exports.set("print", print(lua)?)?;
    exports.set("query", query(lua)?)?;
    Ok(exports)
}

fn cmd(lua: &Lua) -> Result<Table> {
    let exports = lua.create_table()?;
    table! { lua, exports;
        run = |_, (level, cmd, args): (u32, String, Vec<String>)| Ok(crate::cmd::run(level, cmd, args)),
        run_silent = |_, (level, cmd, args): (u32, String, Vec<String>)| Ok(crate::cmd::run_silent(level, cmd, args)),
    }
    Ok(exports)
}

fn display(lua: &Lua) -> Result<Table> {
    let exports = lua.create_table()?;
    table! { lua, exports;
        from_iter = |_, (items, separator): (Vec<String>, String)| Ok(crate::display::from_iter(items, separator).to_string()),
    }
    Ok(exports)
}

fn exit(lua: &Lua) -> Result<Table> {
    let exports = lua.create_table()?;
    table! { lua, exports;
        ok = |_, ()| -> Result<()> { crate::exit::ok() },
        abort = |_, message: String| -> Result<()> { crate::exit::abort(message) },
        err = |_, (code, message): (i32, String)| -> Result<()> { crate::exit::err(code, message) },
    }
    Ok(exports)
}

fn logfile(lua: &Lua) -> Result<Table> {
    let exports = lua.create_table()?;
    table! { lua, exports;
        init = |_, name: String| { crate::logfile::init(name); Ok(()) },
        new_path = |_, name: String| Ok(crate::logfile::new_path(name).display().to_string()),
        handle = |_, ()| Ok(crate::logfile::handle().is_some()),
        write_line = |_, line: String| { crate::logfile::write_line(&line); Ok(()) },
    }
    Ok(exports)
}

fn print(lua: &Lua) -> Result<Table> {
    let exports = lua.create_table()?;
    table! { lua, exports;
        info = |_, (level, message): (u32, String)| { crate::print::info(level, message); Ok(()) },
        dbg = |_, (level, message): (u32, String)| { crate::print::dbg(level, message); Ok(()) },
        warn = |_, (level, message): (u32, String)| { crate::print::warn(level, message); Ok(()) },
        err = |_, (level, message): (u32, String)| { crate::print::err(level, message); Ok(()) },
        prompt = |_, (level, message): (u32, String)| { crate::print::prompt(level, message); Ok(()) },
        ln = |_, ()| { crate::print::ln(); Ok(()) },
    }

    exports.set(
        "INFO_HEADER",
        header(
            lua,
            crate::print::INFO_HEADER.ch,
            crate::print::INFO_HEADER.color,
        )?,
    )?;
    exports.set(
        "DBG_HEADER",
        header(
            lua,
            crate::print::DBG_HEADER.ch,
            crate::print::DBG_HEADER.color,
        )?,
    )?;
    exports.set(
        "WARN_HEADER",
        header(
            lua,
            crate::print::WARN_HEADER.ch,
            crate::print::WARN_HEADER.color,
        )?,
    )?;
    exports.set(
        "ERR_HEADER",
        header(
            lua,
            crate::print::ERR_HEADER.ch,
            crate::print::ERR_HEADER.color,
        )?,
    )?;
    exports.set(
        "PROMPT_HEADER",
        header(
            lua,
            crate::print::PROMPT_HEADER.ch,
            crate::print::PROMPT_HEADER.color,
        )?,
    )?;
    exports.set(
        "Header",
        lua.create_function(|lua, (ch, color): (char, u8)| header(lua, ch, color))?,
    )?;

    Ok(exports)
}

fn header(lua: &Lua, ch: char, color: u8) -> Result<Table> {
    let exports = lua.create_table()?;
    exports.set("ch", ch.to_string())?;
    exports.set("color", color)?;
    exports.set(
        "print",
        lua.create_function(move |_, (level, message): (u32, String)| {
            crate::print::Header { ch, color }.print(level, message);
            Ok(())
        })?,
    )?;
    exports.set(
        "println",
        lua.create_function(move |_, (level, message): (u32, String)| {
            crate::print::Header { ch, color }.println(level, message);
            Ok(())
        })?,
    )?;
    exports.set(
        "print_full",
        lua.create_function(move |_, (level, message): (u32, String)| {
            crate::print::Header { ch, color }.print_full(level, message);
            Ok(())
        })?,
    )?;
    exports.set(
        "println_full",
        lua.create_function(move |_, (level, message): (u32, String)| {
            crate::print::Header { ch, color }.println_full(level, message);
            Ok(())
        })?,
    )?;
    Ok(exports)
}

fn query(lua: &Lua) -> Result<Table> {
    let exports = lua.create_table()?;
    table! { lua, exports;
        line = |_, (level, prompt): (u32, String)| query_result(crate::query::line(level, prompt)),
        num = |_, (level, prompt): (u32, String)| query_result(crate::query::item::<i64>(level, prompt)),
        one = |_, (level, prompt, choices, default): (u32, String, Vec<String>, Option<usize>)| {
            let default = default.map(|index| index.saturating_sub(1));
            query_result(crate::query::one(level, prompt, &choices, default)).map(|index| index + 1)
        },
        many = |_, (level, prompt, choices, defaults): (u32, String, Vec<String>, Vec<usize>)| {
            let defaults = defaults.into_iter().map(|index| index.saturating_sub(1)).collect::<Vec<_>>();
            query_result(crate::query::many(level, prompt, &choices, &defaults))
        },
    }
    Ok(exports)
}

fn query_result<T, E: std::fmt::Display>(result: crate::query::Query<T, E>) -> Result<T> {
    match result {
        crate::query::Query::Ok(value) => Ok(value),
        crate::query::Query::Cancel => crate::exit::abort("Query canceled"),
        crate::query::Query::Quit => crate::exit::abort("Query quit"),
        crate::query::Query::Err(error) => crate::exit::abort(error),
    }
}
