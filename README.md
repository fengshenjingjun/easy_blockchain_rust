# Simple Blockchain in Rust

一个用 Rust 编写的简易区块链实现，用于学习区块链基础原理。包含区块生成、工作量证明（PoW）、区块链验证等功能。

## 功能特性

-   **区块生成**  
    通过哈希链接的区块结构，包含索引、时间戳、数据、前驱哈希和随机数。
-  ⛏️ **工作量证明（PoW）**  
   简单的 SHA-256 哈希碰撞算法，支持可调节的计算难度。
-  🔗 **区块链验证**  
   自动验证区块链的完整性（哈希连续性验证、PoW有效性检查）。
-  🖥️ **基础 CLI 交互**  
   支持命令行创建新区块、打印区块链数据及验证操作。

## 依赖环境

- Rust 1.60+ （[安装指南](https://www.rust-lang.org/tools/install)）
- Cargo（Rust 包管理器）

## 快速开始

### 安装 & 运行

1. 克隆仓库：
   ```bash
   git clone https://github.com/fengshenjingjun/easy_blockchain_rust.git
   cd easy_blockchain_rust
2. 编译并运行：
   ```bash
    cargo run --release

### 项目结构
.
├── Cargo.toml            # 依赖配置
├── src/
│   ├── main.rs           # CLI 入口
│   ├── block.rs          # 区块数据结构
│   ├── pow.rs            # 工作量证明算法
│   ├── cli.rs            # 命令行交互逻辑
│   └── chain.rs          # 区块链管理逻辑
├── README.md             # 项目文档
└── tests/                # 单元测试（可选）