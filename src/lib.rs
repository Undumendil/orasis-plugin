pub struct EventData {
    /// Plugin intended to receive the event. Use None to trigger processing by plugin manager.
    target_id: Option<String>,
    /// A short string describing type of event.
    meta: String,
    /// Optional String receiver.
    message: Option<std::sync::mpsc::Receiver<String>>,
    /// Optional u64 receiver.
    data: Option<std::sync::mpsc::Receiver<u64>>
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

pub trait Emit {
    /// Return value: whether the event was accepted.
    fn emit(ev: Event) -> bool;
}

/// 
pub struct VirtualCanvas {
    width: u64,
    height: u64,
    /// ARGB data; access pixels with [y * width + x].
    data: [u32],

    o_widrh: u64,
    o_height: u64,
    /// Overlay as visible on the screen; access pixels with [y * o_width + x].
    overlay: [char]
}

pub trait ImageEditorPlugin {
    /// Store emitter to be able to send events later.
    fn new<T: ImageEditorPlugin<E: Emit>>(emitter: &E) -> T;
    
    /// Plugin-unique identifier. May be used in `EventData.target_id`.
    fn id() -> String;
    
    /// Plugin is starting to be used.
    fn activate(self, canvas: &VirtualCanvas);
    
    /// Plugin may draw some additional overlay there.
    fn draw(self, canvas: &mut VirtualCanvas);

    /// Return value: whether this plugin consumes the event.
    fn act(self, ev: Event, canvas: &mut VirtualCanvas) -> bool;
    
    /// Plugin is deactivated. It may be activated again later.
    fn deactivate(self);
}
