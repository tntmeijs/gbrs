use std::collections::VecDeque;

/// Contains a single message's data
pub enum MessageData {
    LoadRomFromDisk(String),
    UpdateRunningState(bool),
    TickOnce,
    ResetGameBoy(bool),
}

/// Simple message queue to trigger various actions within the debugger.
///
/// Please note that this object is <b>NOT</b> thread-safe. It simply wraps a [`VecDeque<T>`]
pub struct MessageQueue {
    messages: VecDeque<MessageData>,
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self {
            messages: VecDeque::new(),
        }
    }
}

impl MessageQueue {
    /// Push a message onto the queue
    pub fn push(&mut self, message: MessageData) {
        self.messages.push_back(message);
    }

    /// Poll the queue and pop a message if available
    pub fn poll(&mut self) -> Option<MessageData> {
        self.messages.pop_front()
    }
}
