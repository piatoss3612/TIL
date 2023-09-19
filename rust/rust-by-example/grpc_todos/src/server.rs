use std::{collections::HashMap, pin::Pin, sync::Arc, time::Duration};

use futures::Stream;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Response, Status};

use crate::{todos_server::Todos, Todo, TodoChangeResponse};

pub struct TodoService {
    todos: Arc<Mutex<HashMap<u32, Todo>>>,
}

impl Default for TodoService {
    fn default() -> Self {
        Self {
            todos: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl Todos for TodoService {
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn add(
        &self,
        request: tonic::Request<super::Todo>,
    ) -> Result<Response<TodoChangeResponse>, Status> {
        let todo = request.into_inner();

        let identifier = match todo.id.clone() {
            Some(id) => id,
            None => return Err(Status::invalid_argument("id is required")),
        };

        let mut map = self.todos.lock().await;

        match map.get(&identifier.id) {
            Some(_) => return Err(Status::already_exists("todo already exists")),
            None => {
                map.insert(identifier.id, todo);
                return Ok(Response::new(TodoChangeResponse {
                    id: Some(identifier),
                    message: "todo added".into(),
                }));
            }
        }
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn remove(
        &self,
        request: tonic::Request<super::TodoIdentifier>,
    ) -> Result<Response<TodoChangeResponse>, Status> {
        let request = request.into_inner();
        let mut map = self.todos.lock().await;

        match map.get(&request.id) {
            Some(_) => {
                map.remove(&request.id);
                return Ok(Response::new(TodoChangeResponse {
                    id: Some(request),
                    message: "todo removed".into(),
                }));
            }
            None => return Err(Status::not_found("todo not found")),
        }
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn update(
        &self,
        request: tonic::Request<super::TodoStatusUpdateRequest>,
    ) -> Result<Response<TodoChangeResponse>, Status> {
        let request = request.into_inner();
        let mut map = self.todos.lock().await;

        let identifier = match request.id {
            Some(id) => id,
            None => return Err(Status::invalid_argument("id is required")),
        };

        match map.get_mut(&identifier.id) {
            Some(todo) => {
                todo.status = request.status;
                return Ok(Response::new(TodoChangeResponse {
                    id: Some(identifier),
                    message: "todo updated".into(),
                }));
            }
            None => return Err(Status::not_found("todo not found")),
        }
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn get(
        &self,
        request: tonic::Request<super::TodoIdentifier>,
    ) -> Result<Response<Todo>, Status> {
        let request = request.into_inner();
        let map = self.todos.lock().await;

        match map.get(&request.id) {
            Some(todo) => return Ok(Response::new(todo.clone())),
            None => return Err(Status::not_found("todo not found")),
        }
    }

    #[doc = " Server streaming response type for the Watch method."]
    type WatchStream = Pin<Box<dyn Stream<Item = Result<Todo, Status>> + Send + Sync>>;

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn watch(
        &self,
        request: tonic::Request<super::TodoIdentifier>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        let request = request.into_inner();
        let map = self.todos.lock().await;

        let mut previous_todo = match map.get(&request.id) {
            Some(todo) => todo.clone(),
            None => return Err(Status::not_found("todo not found")),
        };

        let (tx, rx) = mpsc::unbounded_channel();

        let todos = self.todos.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1));

                let map = todos.lock().await;

                let new_todo = match map.get(&request.id) {
                    Some(todo) => todo.clone(),
                    None => {
                        tx.send(Err(Status::not_found("todo not found"))).unwrap();
                        return;
                    }
                };

                if new_todo != previous_todo {
                    tx.send(Ok(new_todo.clone())).unwrap();
                    previous_todo = new_todo;
                }
            }
        });

        let stream = UnboundedReceiverStream::new(rx);

        Ok(Response::new(Box::pin(stream)))
    }
}
