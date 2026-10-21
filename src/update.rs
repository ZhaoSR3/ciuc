use crate::models::{MaintainOrFill, Message, Model, RunningState};

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            model.running_state = RunningState::Done;
        }
        Message::Maintain => {
            model.maintain_or_fill = MaintainOrFill::Maintain;
        }
        Message::Fill => {
            model.maintain_or_fill = MaintainOrFill::Fill;
        }
    }

    None
}
