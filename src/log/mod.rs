use tklog::handle::FileSizeMode;
use tklog::{LEVEL, LogOption};

pub fn init() {
    tklog::LOG.set_option(LogOption {
        level: Some(LEVEL::Debug), // 设置日志级别为 Debug
        console: Some(false),      // 禁用控制台输出
        format: None,              // 使用默认日志格式
        formatter: None,           // 使用默认日志格式化器
        fileoption: Some(Box::new(FileSizeMode::new(
            "fleurs.log", // 日志文件名
            1 << 30,      // 文件大小达到1GB (1<<30字节)时滚动
            1,            // 最多保留10个备份文件
            false,        // 压缩备份文件
        ))),
    });
}
