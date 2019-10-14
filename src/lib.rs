// Base crate for Orasis plugins.
// Copyright (C) 2019 Daniil Fomichev
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

pub struct EventData {
    /// Plugin intended to receive the event. Use `None` to trigger processing by plugin manager.
    pub target_id: Option<String>,
    /// A short string describing type of event.
    pub meta: String,
    /// Optional `String` receiver.
    pub message: Option<std::sync::mpsc::Receiver<String>>,
    /// Optional `u64` receiver.
    pub data: Option<std::sync::mpsc::Receiver<u64>>
}

pub enum Event {
    /// Input event as key presses, mouse clicks, etc.
    Input(termion::event::Event),
    /// Special event for plugin communication.
    Special(EventData),
    /// SIGWYNCH.
    Resize((u16, u16)),
    /// Some time have passed.
    Tick
}

/// Here plugins do their changes.
pub struct VirtualCanvas<'a> {
    pub width: u64,
    pub height: u64,
    /// ARGB data; access pixels with [y * width + x].
    pub data: &'a mut [u32],

    pub o_widrh: u64,
    pub o_height: u64,
    /// Overlay as visible on the screen; access pixels with [y * o_width + x].
    pub overlay: &'a mut [char]
}

pub trait ImageEditorPlugin {
    /// Store emitter to be able to send events later.
    fn setup(emitter: &mut std::sync::mpsc::Sender<Event>);
    
    /// Plugin-unique identifier. May be used in `EventData.target_id`.
    fn id() -> String;
    
    /// Plugin is starting to be used.
    fn activate(&self, canvas: &VirtualCanvas);
    
    /// Plugin may draw some additional overlay there.
    fn draw(&self, canvas: &mut VirtualCanvas);

    /// Return value: whether this plugin consumes the event.
    fn act(&self, ev: Event, canvas: &mut VirtualCanvas) -> bool;
    
    /// Plugin is deactivated. It may be activated again later.
    fn deactivate(&self);
}
