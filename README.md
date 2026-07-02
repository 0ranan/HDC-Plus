# HDC Plus

基于 Tauri v2 + Vue 3 构建的 HarmonyOS 设备管理桌面应用，通过 HDC（HarmonyOS Device Connector）实现画面投屏、HAP 包安装、设备管理等功能。

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri v2 (Rust) |
| 前端 | Vue 3 + TypeScript + Vite |
| 包管理 | pnpm |

## 环境要求

- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/) >= 9
- [Rust](https://www.rust-lang.org/tools/install) (MSVC toolchain，Windows 下需安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022) 并勾选「使用 C++ 的桌面开发」)

## 快速开始

```bash
# 克隆项目
git clone <repo-url>
cd HDC-Plus

# 安装依赖
pnpm install

# 启动开发环境（桌面应用）
pnpm tauri dev
```

## 可用命令

| 命令 | 说明 |
|------|------|
| `pnpm dev` | 启动前端开发服务器（仅浏览器，`http://localhost:1420`） |
| `pnpm build` | 类型检查并构建前端产物 |
| `pnpm preview` | 预览前端构建结果 |
| `pnpm tauri dev` | 启动 Tauri 桌面应用（开发模式，含热更新） |
| `pnpm tauri build` | 构建生产版本桌面安装包 |

## 项目结构

```
HDC-Plus/
├── index.html                    # 入口 HTML
├── package.json                  # 前端依赖与脚本
├── pnpm-lock.yaml
├── vite.config.ts                # Vite 配置
├── tsconfig.json                 # TypeScript 配置
├── src/                          # Vue 前端源码
│   ├── main.ts                   # 应用入口
│   ├── App.vue                   # 根组件
│   ├── style.css                 # 全局样式
│   ├── assets/                   # 静态资源
│   └── components/               # Vue 组件
└── src-tauri/                    # Tauri / Rust 后端
    ├── Cargo.toml                # Rust 依赖配置
    ├── tauri.conf.json           # Tauri 应用配置
    ├── build.rs                  # 构建脚本
    ├── capabilities/             # 权限声明
    └── src/
        ├── main.rs               # Rust 入口
        └── lib.rs                # Tauri 应用初始化
```

## 功能规划

- [ ] 设备发现与连接管理
- [ ] 画面投屏（screen mirroring）
- [ ] HAP 包安装与卸载
- [ ] 文件传输
- [ ] 设备信息查看
- [ ] Shell 命令执行
- [ ] 日志查看
