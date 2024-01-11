mod translate;

pub mod codes;
pub use codes::{V39Pad, V39Key};

use once_cell::sync::OnceCell;
use std::sync::Mutex;
use crate::prelude::*;


static INSTANCE: OnceCell<InputManager> = OnceCell::new();


pub(crate) struct InputManager
{
    keys_down: Mutex<Vec<V39Key>>,
    keys_up: Mutex<Vec<V39Key>>,
    keys_held: Mutex<Vec<V39Key>>,
    snapshot_keys_down: Mutex<Vec<V39Key>>,
    snapshot_keys_up: Mutex<Vec<V39Key>>,
    snapshot_keys_held: Mutex<Vec<V39Key>>,
}


impl InputManager
{
    pub(crate) fn init() -> V39Result<&'static InputManager>
    {
        let input_manager = InputManager {
            keys_down: Mutex::new(vec![]),
            keys_up: Mutex::new(vec![]),
            keys_held: Mutex::new(vec![]),
            snapshot_keys_down: Mutex::new(vec![]),
            snapshot_keys_up: Mutex::new(vec![]),
            snapshot_keys_held: Mutex::new(vec![]),
        };
        
        if INSTANCE.set(input_manager).is_err()
        {
            return Err(V39Error::Reinit("Input Manager".into()));
        }

        Ok(INSTANCE.get().unwrap())
    }

    pub(crate) fn get() -> &'static InputManager
    {
        INSTANCE.get().expect("Input Manager was uninitialized")
    }

    pub(crate) fn push_key_down(&self, key: V39Key)
    {
        if let Ok(mut keys_down) = self.keys_down.lock()
        {
            keys_down.push(key);
        }
    }

    pub(crate) fn push_key_up(&self, key: V39Key)
    {
        if let Ok(mut keys_up) = self.keys_up.lock()
        {
            keys_up.push(key);
        }
    }

    pub(crate) fn snapshot_keys_down(&self)
    {
        if let (Ok(mut keys), Ok(mut snapshot)) 
            = (self.keys_down.lock(), self.snapshot_keys_down.lock())
        {
            *snapshot = keys.drain(..).collect::<Vec<_>>();
        }
    }

    pub(crate) fn snapshot_keys_up(&self)
    {
        if let (Ok(mut keys), Ok(mut snapshot)) 
            = (self.keys_up.lock(), self.snapshot_keys_up.lock())
        {
            *snapshot = keys.drain(..).collect::<Vec<_>>();
        }
    }

    pub(crate) fn snapshot_keys_held(&self)
    {
        if let (Ok(mut keys), Ok(mut snapshot)) 
            = (self.keys_held.lock(), self.snapshot_keys_held.lock())
        {
            *snapshot = keys.drain(..).collect::<Vec<_>>();
        }
    }

    pub(crate) fn apply_down_snapshot(&self)
    {
        if let (Ok(mut keys), Ok(mut snapshot)) 
            = (self.keys_down.lock(), self.snapshot_keys_down.lock())
        {
            keys.extend(snapshot.drain(..));
        }
    }

    pub(crate) fn apply_up_snapshot(&self)
    {
        if let (Ok(mut keys), Ok(mut snapshot)) 
            = (self.keys_up.lock(), self.snapshot_keys_up.lock())
        {
            keys.extend(snapshot.drain(..));
        }
    }

    pub(crate) fn apply_held_snapshot(&self)
    {
        if let (Ok(mut keys), Ok(mut snapshot)) 
            = (self.keys_held.lock(), self.snapshot_keys_held.lock())
        {
            keys.extend(snapshot.drain(..));
        }
    }

    pub(crate) fn down_snapshot_contains(&self, key: V39Key) -> bool
    {
        if let Ok(keys) = self.snapshot_keys_down.lock()
        {
            return keys.contains(&key);
        }

        false
    }

    pub(crate) fn up_snapshot_contains(&self, key: V39Key) -> bool
    {
        if let Ok(keys) = self.snapshot_keys_up.lock()
        {
            return keys.contains(&key);
        }

        false
    }

    pub(crate) fn held_snapshot_contains(&self, key: V39Key) -> bool
    {
        if let Ok(keys) = self.snapshot_keys_held.lock()
        {
            return keys.contains(&key);
        }

        false
    }

    pub(crate) fn clear_down_snapshot(&self)
    {
        self.snapshot_keys_down
            .lock()
            .unwrap()
            .clear();
    }

    pub(crate) fn clear_up_snapshot(&self)
    {
        self.snapshot_keys_down
            .lock()
            .unwrap()
            .clear();
    }

    pub(crate) fn clear_held_snapshot(&self)
    {
        self.snapshot_keys_up
            .lock()
            .unwrap()
            .clear();
    }

    pub(crate) fn down_held_up_conversion(&self)
    {
        let mut held = self.snapshot_keys_held.lock().unwrap();
        let mut up = self.snapshot_keys_up.lock().unwrap();
        let mut down = self.snapshot_keys_down.lock().unwrap();
 
        let mut new_held = vec![];

        for key in down.iter()
        {
            if !held.contains(key) 
            {
                new_held.push(*key)
            }
        }

        while let Some(key) = held.pop()
        {
            if !up.contains(&key)
            {
                new_held.push(key);
            }
        }

        *held = new_held;
        up.clear()
    }
}

