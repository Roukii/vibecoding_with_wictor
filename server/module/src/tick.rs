use spacetimedb::{reducer, table, ReducerContext, ScheduleAt, Table, TimeDuration, Timestamp};

#[table(name = game_tick, scheduled(tick))]
pub struct GameTick {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub scheduled_at: ScheduleAt,
    pub last_tick_time: Timestamp,
}

// Tick rate: 20 times per second = 50ms interval
const TICK_INTERVAL_MICROS: i64 = 50_000; // 50ms in microseconds

#[reducer]
pub fn tick(ctx: &ReducerContext, schedule: GameTick) -> Result<(), String> {
    // Only allow the module to call this reducer (security check)
    if ctx.sender != ctx.identity() {
        return Err("Tick reducer can only be called by the module itself".to_string());
    }

    let current_time = ctx.timestamp;
    let last_tick_time = schedule.last_tick_time;

    // Calculate deltaTime in seconds
    // let delta_time_micros =
    //     current_time.to_micros_since_unix_epoch() - last_tick_time.to_micros_since_unix_epoch();

    // Update the tick schedule with the new time
    let updated_schedule = GameTick {
        id: schedule.id,
        scheduled_at: schedule.scheduled_at, // Keep the same interval
        last_tick_time: current_time,
    };

    ctx.db.game_tick().id().update(updated_schedule);

    // Here you can add your game logic that should run every tick
    // For example:
    // - Update entity positions
    // - Process physics
    // - Handle AI
    // - Check for collisions
    // etc.

    Ok(())
}

// Helper function to initialize the tick system (called from init reducer)
pub fn initialize_tick_system(ctx: &ReducerContext) -> Result<(), String> {
    let current_time = ctx.timestamp;
    let tick_interval = TimeDuration::from_micros(TICK_INTERVAL_MICROS);

    // Create the initial tick schedule
    let tick_schedule = GameTick {
        id: 0,                              // auto_inc will assign this
        scheduled_at: tick_interval.into(), // Convert to ScheduleAt::Interval
        last_tick_time: current_time,
    };

    ctx.db.game_tick().insert(tick_schedule);

    log::info!("Tick system initialized - running at 20 Hz (every 50ms)");
    Ok(())
}
