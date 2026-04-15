use crate::messages::Message;
#[derive(Default)]
pub struct Counter {
    pub value: i64,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Counter;
    use crate::messages::Message;

    #[test]
    fn increments_value() {
        let mut counter = Counter { value: 0 };
        counter.update(Message::Increment);
        assert_eq!(counter.value, 1);
    }

    #[test]
    fn decrements_value() {
        let mut counter = Counter { value: 0 };
        counter.update(Message::Decrement);
        assert_eq!(counter.value, -1);
    }

    #[test]
    fn applies_multiple_updates() {
        let mut counter = Counter { value: 10 };
        counter.update(Message::Increment);
        counter.update(Message::Increment);
        counter.update(Message::Decrement);
        assert_eq!(counter.value, 11);
    }
}
