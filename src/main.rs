use rand::SeedableRng;
use thiserror::Error;

struct Client;

struct ConnectorConfig;

struct Lock {
    pub key: String
}

trait RedisConnector {
    fn acquire_lock(&self) -> Result<Lock, LockError>;
    fn release_lock(&self, lock: Lock) -> Result<(), LockError>;
}

#[derive(Error, Debug)]
enum LockError {
    #[error("lock being holded by another client")]
    HoldedByAnotherClient,
    #[error("you dont own this lock anymore")]
    InvalidKey,
    #[error("lock doesn't exists anymore")]
    DoesNotExists,
}

struct Connector;

impl RedisConnector for Connector {
    fn acquire_lock(&self) -> Result<Lock, LockError> {
        let mut rng = rand::rngs::StdRng::from_seed();
        todo!()
    }

    fn release_lock(&self, lock: Lock) -> Result<(), LockError> {
        todo!()
    }
}

fn main() {
    let connector = Connector {};

    let lock = connector.acquire_lock()
}

#[cfg(test)]
mod tests {
    fn test_only_one_client_can_hold_a_lock() {
        
    }
}

