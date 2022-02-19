use crate::*;

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hmget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        self.keys
            .iter()
            .map(|key| match store.get(&self.table, key) {
                Ok(Some(v)) => v,
                _ => Value::default(),
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl CommandService for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(v) => match store.set(&self.table, v.key, v.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => Value::default().into(),
        }
    }
}

impl CommandService for Hmset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let table = self.table;
        self.pairs
            .into_iter()
            .map(|pair| match store.set(&table, pair.key, pair.value.unwrap_or_default()) {
                Ok(Some(v)) => v,
                _ => Value::default(),
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl CommandService for Hdel {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.del(&self.table, self.key.as_str()) {
            Ok(Some(v)) => v.into(),
            Ok(None) => Value::default().into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hmdel {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        self.keys
            .iter()
            .map(|key| match store.del(&self.table, key.as_str()) {
                Ok(Some(v)) => v.into(),
                _ => Value::default(),
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl CommandService for Hexists {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.contains(&self.table, self.key.as_str()) {
            Ok(v) => v.into(),
            Err(e) => e.into(), 
        }
    }
}

impl CommandService for Hmexists {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let table = self.table;
        self.keys
            .into_iter()
            .map(|key| match store.contains(table.as_str(), key.as_str()) {
                Ok(v) => v.into(),
                _ => Value::default(), 
            })
            .collect::<Vec<_>>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_request::RequestData;

    #[test]
    fn hset_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[Value::default()], &[]);
    }

    #[test]
    fn hmset_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hmset(
            "t1", 
            vec![Kvpair::new("hello", "world".into()), Kvpair::new("hello", "natsu".into())]
        );
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[Value::default(), "world".into()], &[]);
    }

    #[test]
    fn hget_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("score", "u1", 10.into());
        dispatch(cmd, &store);
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[10.into()], &[]);
    }

    #[test]
    fn hget_with_none_exist_key_should_return_404() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_error(res, 404, "Not found");
    }

    #[test]
    fn hmget_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hmset(
            "score", 
            vec![Kvpair::new("u1", 10.into()), Kvpair::new("u2", 8.into())]
        );
        dispatch(cmd, &store);
        let cmd = CommandRequest::new_hmget("score", vec!["u1".to_owned(), "u2".to_owned()]);
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[10.into(), 8.into()], &[]);
    }

    #[test]
    fn hgetall_should_work() {
        let store = MemTable::new();
        let cmds = vec![
            CommandRequest::new_hset("score", "u1", 10.into()),
            CommandRequest::new_hset("score", "u2", 8.into()),
            CommandRequest::new_hset("score", "u3", 11.into()),
            CommandRequest::new_hset("score", "u1", 6.into()),
        ];
        for cmd in cmds {
            dispatch(cmd, &store);
        }

        let cmd = CommandRequest::new_hgetall("score");
        let res = dispatch(cmd, &store);
        let pairs = &[
            Kvpair::new("u1", 6.into()),
            Kvpair::new("u2", 8.into()),
            Kvpair::new("u3", 11.into()),
        ];
        assert_res_ok(res, &[], pairs);
    }

    #[test]
    fn hdel_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("score", "u1", 30.into());
        dispatch(cmd, &store);
        let cmd = CommandRequest::new_hdel("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[30.into()], &[]);
    }

    #[test]
    fn hmdel_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hmset(
            "score",
            vec![Kvpair::new("u1", 10.into()), Kvpair::new("u2", 8.into())]
        );
        dispatch(cmd, &store);
        let cmd = CommandRequest::new_hmdel(
            "score", 
            vec!["u1".to_owned(), "u2".to_owned()]
        );
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[10.into(), 8.into()], &[]);
    }

    #[test]
    fn hexists_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("score", "u1", "10".into());
        dispatch(cmd, &store);

        let cmd = CommandRequest::new_hexists("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[true.into()], &[]);

        let cmd = CommandRequest::new_hexists("score", "u2");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[false.into()], &[]);
    }

    #[test]
    fn hmexists_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset(
            "score",
            "u1", 10.into()
        );
        dispatch(cmd, &store);
        let cmd = CommandRequest::new_hmexists(
            "score",
            vec!["u1".to_owned(), "u2".to_owned()]
        );
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[true.into(), false.into()], &[]);
    }

    fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
        match cmd.request_data.unwrap() {
            RequestData::Hget(v) => v.execute(store),
            RequestData::Hmget(v) => v.execute(store),
            RequestData::Hgetall(v) => v.execute(store),
            RequestData::Hset(v) => v.execute(store),
            RequestData::Hmset(v) => v.execute(store),
            RequestData::Hdel(v) => v.execute(store),
            RequestData::Hmdel(v) => v.execute(store),
            RequestData::Hexists(v) => v.execute(store),
            RequestData::Hmexists(v) => v.execute(store),
            _ => unimplemented!(),
        }
    }

    // 测试成功返回的结果
    fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
        res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(res.status, 200);
        assert_eq!(res.message, "");
        assert_eq!(res.values, values);
        assert_eq!(res.pairs, pairs);
    }

    // 测试失败返回的结果
    fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {
        assert_eq!(res.status, code);
        assert!(res.message.contains(msg));
        assert_eq!(res.values, &[]);
        assert_eq!(res.pairs, &[]);
    }
}