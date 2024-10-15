use iced::Task;
use std::collections::HashMap;
use std::sync::Arc;

use crate::blockchain::AlloyClient;

pub trait FormTransactionHandler<T> {
    fn handle_transaction(client: Arc<AlloyClient>, data: &HashMap<String, String>) -> Task<T>;
}
