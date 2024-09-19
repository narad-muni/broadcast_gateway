use super::Output;

struct KafkaOutput {}

impl Output for KafkaOutput {
    fn write<T>(&self, data: T) {
        
    }
}