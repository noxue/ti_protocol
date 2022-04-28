use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use ti_macro_derive::{TiPack, TiUnPack};

pub trait TiPack {
    fn pack(&self) -> Result<Vec<u8>, String>;
}

pub trait TiUnPack {
    fn unpack<'a>(encoded: &'a [u8]) -> Result<Self, String>
    where
        Self: Deserialize<'a>;
}

/// 序列化，传入结构体，返回（长度+结构体）序列化后的数据
fn pack<T>(t: &T) -> Result<Vec<u8>, String>
where
    T: Serialize,
{
    match bincode::serialize(&t) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{:?}", e)),
    }
}

/// 反序列化，去除数据包头部长度后的数据
fn unpack<'a, T>(encoded: &'a [u8]) -> Result<T, String>
where
    T: Deserialize<'a>,
{
    match bincode::deserialize(encoded) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{:?}", e)),
    }
}

/// 获取数据包头部长度
pub fn get_header_size() -> usize {
    PacketHeader::new(0, PackType::Task).pack().unwrap().len()
}

/// 数据包类型
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PackType {
    Task,
    TaskResult,
}

/// 数据头
#[derive(Serialize, Deserialize, PartialEq, Debug, TiPack, TiUnPack)]
pub struct PacketHeader {
    pub flag: u16, // 标志位
    pub body_size: u64,
    pub pack_type: PackType, // 数据包类型
}

impl PacketHeader {
    fn new(body_size: u64, pack_type: PackType) -> PacketHeader {
        PacketHeader {
            flag: 0x55aa,
            body_size,
            pack_type,
        }
    }

    // 检测标志位是否正确
    pub fn check_flag(&self) -> bool {
        self.flag == 0x55aa
    }
}

/// 数据头
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Packet {
    pub header: PacketHeader,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn new<T>(data_type: PackType, data: T) -> Result<Packet, String>
    where
        T: Serialize + TiPack + TiUnPack,
    {
        let data = data.pack()?;
        Ok(Packet {
            header: PacketHeader::new(data.len() as u64, data_type),
            data,
        })
    }
}

impl TiPack for Packet {
    fn pack(&self) -> Result<Vec<u8>, String> {
        let mut data = self.header.pack()?;
        data.extend_from_slice(&self.data);
        Ok(data)
    }
}

/// 获取数据
#[derive(Serialize, Deserialize, PartialEq, Debug, TiPack, TiUnPack)]
pub struct Task {
    pub task_id: i32,         // 任务id，随机生成
    pub product_name: String, // 产品名
}

impl Task {
    pub fn new(task_id: i32, product_name: String) -> Task {
        Task {
            task_id,
            product_name,
        }
    }
}

/// worker执行完任务后返回执行结果
#[derive(Serialize, Deserialize, PartialEq, Debug, TiPack, TiUnPack)]
pub struct TaskResult {
    pub task_id: i32,                // 任务id
    pub result: Result<i32, String>, // 执行结果
}

// new
impl TaskResult {
    pub fn new(task_id: i32, result: Result<i32, String>) -> TaskResult {
        TaskResult { task_id, result }
    }
}

#[cfg(test)]
mod tests {
    use crate::{PackType, Task, TaskResult, TiPack, TiUnPack};

    #[test]
    fn test_task_result() {
        let task_result = TaskResult {
            task_id: 10,
            result: Ok(1000),
        };
        let encoded = task_result.pack().unwrap();
        let decoded = TaskResult::unpack(&encoded[8..]).unwrap();
        assert_eq!(decoded, decoded);
    }

    //测试task
    #[test]
    fn test_task() {
        let task = Task {
            task_id: 10,
            product_name: "test".to_string(),
        };
        let encoded: Vec<u8> = bincode::serialize(&task).unwrap();
        let decoded: Task = bincode::deserialize(&encoded[..]).unwrap();
        assert_eq!(task, decoded);
    }

    //测试 PackType
    #[test]
    fn test_pack_type() {
        let task = PackType::TaskResult;
        let encoded: Vec<u8> = bincode::serialize(&task).unwrap();
        let decoded: PackType = bincode::deserialize(&encoded[..]).unwrap();
        println!("{:?}", encoded);
    }
}
