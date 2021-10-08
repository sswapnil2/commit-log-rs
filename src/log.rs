use std::sync::{Mutex};
use bytes::Bytes;

#[derive(Debug)]
pub struct Record {
    pub value: Bytes,
    pub offset: i64
}

impl Record {
    pub fn new(value: Bytes) -> Self {
        Record {
            value,
            offset: 0
        }
    }
}

impl Clone for Record {
    fn clone(&self) -> Self {
       Record {
           value: self.value.clone(),
           offset: self.offset
       }
    }
}


pub struct Log {
    records: Mutex<Vec<Record>>
}

impl Log {
    pub fn new() -> Self {
        Log {
            records: Mutex::new(vec![])
        }
    }

    pub fn append(&self, mut record: Record) -> Option<i64> {
        let mut records = self.records.lock().unwrap();

        let new_offset = records.len() as i64;

        record.offset = new_offset;

        records.push( record);

        Some(new_offset)
    }

    pub fn read(&self, offset: i64) -> Result<Record, &'static str> {
        let records = self.records.lock().unwrap();

        if offset >= records.len() as i64 {
            return Err("Offset not found");
        }
        let new_offset = (offset as usize).clone();
        let record = &records[new_offset];
        let cloned = record.clone();
        Ok(cloned)
    }

    pub fn len(&self) -> usize {
        self.records.lock().unwrap().len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_record() {
        let record = Record::new("Hello".into());
        assert_eq!(record.offset, 0);
        assert_eq!(record.value, Bytes::from("Hello"))
    }

    #[test]
    fn test_clone_for_record(){
        let record = Record::new("Hello".into());
        let cloned = record.clone();
        assert_eq!(cloned.offset, 0);
        assert_eq!(cloned.value, Bytes::from("Hello"))
    }

    #[test]
    fn test_new_log_creation(){
        let log = Log::new();
        assert_eq!(log.len(), 0);
    }

    #[test]
    fn test_add_new_record(){
        let log = Log::new();
        let record = Record::new("h".into());
        let offset = log.append(record).unwrap();
        assert_eq!(offset, 0);

        let r1 = log.read(0).unwrap();
        assert_eq!(r1.value, Bytes::from("h"));


        let record = Record::new("3".into());
        let offset = log.append(record).unwrap();
        assert_eq!(offset, 1);

        let r1 = log.read(1).unwrap();
        assert_eq!(r1.value, Bytes::from("3"));
    }

}