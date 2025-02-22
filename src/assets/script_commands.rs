pub enum ScriptCommands {
    OpenDoor,     // Takes entity's map ID, non-blocking.
    CloseDoor,    // Takes entity's map ID, non-blocking.
    ToggleDoor,   // Takes entity's map ID, non-blocking.
    MoveTo,       // Takes waypoint's map ID, blocking.
    LookAt,       // Takes waypoint's map ID, blocking.
    Wait,         // Takes delay in seconds, blocking.
    Repeat,       // non-blocking.
    PlayAudio,    // Sound handle and text, non-blocking.
    TickCount,    // Something to do with action synchronization?
    Sync,         // blocking, blocks until both MoveTo and LookAt complete?
    Reset,        // Used to reset floor switches to their off state.
    SetDoorTimer, // Shows door timer UI, takes countdown time, non-blocking.
}
