# 任务列表: HDC Plus - 设备发现与连接管理 (P1)

**输入**: 设计文档来自 `/specs/002-hdc-plus/`
**前置条件**: spec.md（用户故事与验收场景）、constitution.md（项目准则）

**范围**: 仅 User Story 1 —— 设备发现与连接管理（USB / 局域网）。其余 P2、P3 用户故事不在本次任务范围内。

**技术栈**: Tauri v2 (Rust) + Vue 3 + TypeScript + Pinia + Element Plus

---

## 格式说明: `[ID] [P?] [Story?] 描述`

- **[P]**: 可并行执行（不同文件，无依赖）
- **[US1]**: 属于 User Story 1（设备连接与管理）
- 每条任务包含具体文件路径

---

## Phase 1: 项目初始化（Setup）

**目标**: 安装项目依赖，搭建基础目录结构

- [x] T001 安装前端依赖 Pinia 和 Element Plus：`pnpm add pinia element-plus @element-plus/icons-vue`
- [x] T002 [P] 注册 Element Plus（完整导入或按需导入）和 Pinia 实例到 `src/main.ts`
- [x] T003 [P] 创建项目目录结构：
  - `src/stores/`（Pinia 状态管理）
  - `src/types/`（TypeScript 类型定义）
  - `src/utils/`（工具函数）
  - `src-tauri/src/hdc/`（HDC 客户端模块）
  - `src-tauri/src/models/`（Rust 数据模型）
  - `src-tauri/src/commands/`（Tauri 命令模块）
- [x] T004 [P] 删除模板示例文件 `src/components/HelloWorld.vue` 和 `src/assets/hero.png`、`src/assets/vite.svg`、`src/assets/vue.svg`

---

## Phase 2: 基础设施 —— Rust 后端（Foundational）

**目标**: 搭建 HDC 命令执行能力与设备数据模型，所有用户故事的前置依赖。

**⚠️ 关键**: 此阶段完成后，Rust 后端即可被任何前端组件通过 IPC 调用。

- [x] T005 [P] 定义设备数据模型 `Device`（含 Serialize/Deserialize）在 `src-tauri/src/models/device.rs`：
  - 字段：`id`（设备标识）、`name`、`model`、`os_version`、`sn`、`connection_type`（USB/WiFi）、`status`（online/offline）、`battery_level`
  - 同时创建 `src-tauri/src/models/mod.rs` 导出该模块
- [x] T006 实现 HDC 命令行客户端在 `src-tauri/src/hdc/client.rs`：
  - 函数 `find_hdc_binary()` → 在系统 PATH 中定位 `hdc` 可执行文件路径
  - 函数 `execute_hdc_command(args: &[&str]) -> Result<String>` → 异步执行 HDC 命令并返回标准输出
  - 函数 `parse_device_list(output: &str) -> Vec<Device>` → 解析 `hdc list targets` 输出为设备列表
  - 函数 `parse_device_info(output: &str) -> Device` → 解析 `hdc shell getprop` 等输出补充设备详情
  - 错误处理：HDC 未找到、命令执行超时（10 秒）、输出解析失败
  - 同时创建 `src-tauri/src/hdc/mod.rs` 导出该模块
- [x] T007 实现 Tauri 命令在 `src-tauri/src/commands/device.rs`：
  - `#[tauri::command] list_devices() -> Result<Vec<Device>, String>` → 列出所有已连接设备
  - `#[tauri::command] connect_device(ip: String, port: u16) -> Result<Device, String>` → 通过网络 IP 连接设备
  - `#[tauri::command] disconnect_device(id: String) -> Result<(), String>` → 断开指定设备连接
  - `#[tauri::command] get_device_info(id: String) -> Result<Device, String>` → 获取指定设备详细信息
  - 同时创建 `src-tauri/src/commands/mod.rs` 导出该模块
- [x] T008 在 `src-tauri/src/lib.rs` 中注册 Tauri 命令并初始化应用状态：
  - 在 `run()` 函数中通过 `.manage()` 注入 HDC 客户端实例作为 Tauri 状态
  - 通过 `.invoke_handler(tauri::generate_handler![...])` 注册所有设备相关命令
  - 导入新增的 models 和 commands 模块

---

## Phase 3: 基础设施 —— Vue 前端（Foundational）

**目标**: 搭建前端类型定义、API 封装层和状态管理，所有前端组件的共享基座。

**⚠️ 关键**: 此阶段完成后，Vue 组件即可调用后端命令并管理设备状态。

- [x] T009 [P] 定义 TypeScript 设备接口在 `src/types/device.ts`：
  - 接口 `Device`：与 Rust `Device` 结构体字段一一对应
  - 类型 `ConnectionType = 'USB' | 'WiFi'`
  - 类型 `DeviceStatus = 'online' | 'offline'`
- [x] T010 [P] 创建 Tauri IPC 封装工具在 `src/utils/tauri.ts`：
  - 函数 `invokeListDevices(): Promise<Device[]>`
  - 函数 `invokeConnectDevice(ip: string, port: number): Promise<Device>`
  - 函数 `invokeDisconnectDevice(id: string): Promise<void>`
  - 函数 `invokeGetDeviceInfo(id: string): Promise<Device>`
  - 统一错误处理：捕获 Tauri invoke 错误，转换为中文提示信息
- [x] T011 创建 Pinia 设备状态存储在 `src/stores/device.ts`：
  - 状态：`devices: Device[]`、`selectedDeviceId: string | null`、`loading: boolean`、`errorMessage: string | null`
  - 动作：`fetchDevices()`（调用 list_devices）、`addDevice(ip, port)`（调用 connect_device）、`removeDevice(id)`（调用 disconnect_device）、`refreshDeviceInfo(id)`（调用 get_device_info）
  - Getter：`onlineDevices`、`offlineDevices`、`selectedDevice`、`deviceCount`
  - 错误处理：捕获异常并设置 `errorMessage` 为中文提示

---

## Phase 4: User Story 1 —— 设备连接与管理（Priority: P1）🎯 MVP

**目标**: 用户可以自动发现 USB 设备、手动连接网络设备、查看设备信息、监控连接状态。

**独立测试**: 通过 USB 连接一台 OpenHarmony 设备，启动应用后设备出现在列表中并显示基本信息；断开 USB 后设备状态更新为离线。

### UI 组件实现

- [x] T012 [P] [US1] 实现设备卡片组件在 `src/components/DeviceCard.vue`：
  - 显示设备名称、型号、连接方式图标（USB 线图标 / WiFi 图标）
  - 在线设备显示绿色状态指示点，离线设备显示灰色状态指示点
  - 选中态高亮样式（蓝色边框）
  - 点击卡片触发 `$emit('select', deviceId)`
  - Props: `device: Device`、`isSelected: boolean`
- [x] T013 [P] [US1] 实现设备详情面板组件在 `src/components/DeviceDetail.vue`：
  - 显示完整设备信息：名称、型号、操作系统版本、SN 序列号、连接方式、电池电量、状态
  - 显示"断开连接"按钮（仅网络连接设备可操作）
  - 显示"刷新信息"按钮
  - Props: `device: Device | null`
  - Emits: `disconnect(deviceId)`、`refresh(deviceId)`
- [x] T014 [US1] 实现设备列表主视图组件在 `src/components/DeviceList.vue`：
  - 顶部工具栏：标题"设备列表" + 设备总数 + "添加设备"按钮
  - 左侧设备卡片列表（使用 DeviceCard 组件，支持滚动）
  - 右侧设备详情面板（使用 DeviceDetail 组件，选中设备时显示）
  - 空状态提示：无已连接设备时显示引导文字"暂无已连接设备，请通过 USB 连接设备或点击「添加设备」按钮手动连接"
  - 加载状态：数据获取中显示 Element Plus 骨架屏或加载动画
  - 错误提示：使用 Element Plus `ElMessage` 显示错误信息
- [x] T015 [P] [US1] 实现手动连接对话框组件在 `src/components/ConnectDialog.vue`：
  - 使用 Element Plus `el-dialog` + `el-form`
  - 输入字段：IP 地址（带格式校验）、端口号（默认 5555）
  - "测试连接"按钮 → 调用 connect_device，成功后关闭对话框
  - "取消"按钮关闭对话框
  - 连接中显示加载状态，连接失败显示错误提示
  - Props: `visible: boolean`
  - Emits: `close`、`connected(device: Device)`

### 应用集成

- [x] T016 [US1] 在 `src/App.vue` 中集成所有组件与状态管理：
  - 在 `onMounted` 中调用 `deviceStore.fetchDevices()` 初始加载设备列表
  - 挂载 DeviceList 组件作为主视图
  - 管理 ConnectDialog 的显示/隐藏状态
  - 响应设备选中、断开连接、刷新信息等事件
  - 响应 `deviceStore.errorMessage` 变化并显示错误提示
- [x] T017 [US1] 实现设备列表自动轮询在 `src/stores/device.ts` 中：
  - 添加 `startPolling(intervalMs: number)` 动作：定时调用 `fetchDevices()`
  - 添加 `stopPolling()` 动作：清除定时器
  - 在 `src/App.vue` 的 `onMounted` 中启动轮询（间隔 3 秒），`onUnmounted` 中停止
  - 检测设备状态变化：对比前后两次设备列表，发现设备离线时通过 `ElNotification` 提示用户
- [x] T018 [US1] 添加连接状态视觉反馈在 `src/style.css` 或各组件 `<style>` 中：
  - 在线设备卡片：正常样式 + 绿色左边框
  - 离线设备卡片：灰色半透明样式 + 灰色左边框
  - 状态切换过渡动画（opacity 渐变 0.3s）
  - Element Plus 主题变量覆盖（主色调整为项目品牌色）
- [x] T019 [US1] 实现设备断连处理：
  - 当前选中的设备断连时，自动清除选中状态，详情面板显示"该设备已断开连接"
  - 断连时通过 `ElNotification` 弹出系统通知，内容包含设备名称和断连原因提示

---

## Phase 5: 打磨与收尾（Polish）

**目标**: 完善边缘情况处理与用户体验细节

- [x] T020 添加全局错误处理与中文提示：
  - HDC 未找到：提示"未检测到 HDC 工具，请确保 HDC 已安装并添加到系统 PATH 环境变量中"
  - 设备连接失败：提示"设备连接失败：[具体错误原因]"
  - 设备断开失败：提示"断开连接失败：[具体错误原因]"
  - 网络超时：提示"连接超时，请检查设备 IP 地址和网络状态"
- [x] T021 验证独立测试场景（与 spec.md 验收场景对齐）：
  - ✅ USB 设备自动检测 → 设备出现在列表
  - ✅ 设备断连 → 状态实时更新为离线，视觉置灰
  - ✅ 手动 IP 连接 → 设备出现在列表并标记"网络连接"
  - ✅ 点击设备 → 展示完整设备信息
- [x] T022 运行 `pnpm tauri dev` 验证应用可正常启动、前端与 Rust 后端 IPC 通信正常

---

## 依赖关系与执行顺序

### 阶段依赖

- **Phase 1（Setup）**: 无依赖 —— 可立即开始
- **Phase 2（Rust 后端）**: 依赖 Phase 1 完成 —— 阻塞所有用户故事
- **Phase 3（Vue 前端）**: 依赖 Phase 1 完成 —— 阻塞所有用户故事
- **Phase 4（US1）**: 依赖 Phase 2 和 Phase 3 完成
- **Phase 5（Polish）**: 依赖 Phase 4 完成

### 用户故事内依赖

- T012 DeviceCard、T013 DeviceDetail、T015 ConnectDialog 可并行开发（无相互依赖）
- T014 DeviceList 依赖 T012（使用 DeviceCard）
- T016 App.vue 集成依赖所有组件完成
- T017 轮询依赖 T011（Pinia store）已有 `fetchDevices` 动作
- T018 样式依赖 T012、T013 组件结构已确定
- T019 断连处理依赖 T011（Pinia store）和 T017（轮询机制）

### 并行执行机会

```bash
# Phase 1 并行任务：
Task: "T002 注册 Element Plus 和 Pinia"
Task: "T003 创建目录结构"
Task: "T004 删除模板文件"

# Phase 2 + Phase 3 可并行（不同语言，无交叉依赖）：
# Rust 后端（Phase 2）:
Task: "T005 定义 Device 模型"
# Vue 前端（Phase 3）:
Task: "T009 定义 TypeScript 接口"
Task: "T010 创建 Tauri IPC 封装"

# Phase 4 组件层并行：
Task: "T012 DeviceCard 组件"
Task: "T013 DeviceDetail 组件"  
Task: "T015 ConnectDialog 组件"
```

---

## 实现策略

### MVP 优先（仅 User Story 1）

1. 完成 Phase 1: 项目初始化
2. 完成 Phase 2: Rust 后端基础（HDC 客户端 + Tauri 命令）
3. 完成 Phase 3: Vue 前端基础（类型 + API + 状态管理）
4. **临界点**: 此时可通过 Rust 测试或 Tauri dev 验证 HDC 命令调用是否正常
5. 完成 Phase 4: User Story 1 全部组件与集成
6. **验证**: 使用真实 OpenHarmony 设备通过 USB 连接，验证自动发现、信息展示、断连检测
7. 完成 Phase 5: 打磨与收尾

### 增量交付

- Phase 1 → 基础项目框架就绪
- Phase 2 + 3 → 前后端 IPC 通道打通，可进行命令行级别验证
- Phase 4 → **MVP 可用**: 设备列表显示、连接管理、信息查看全部就绪
- Phase 5 → 生产级质量，错误处理与用户体验完善

---

## 备注

- [P] 标记的任务 = 不同文件、无依赖、可并行
- [US1] 标签将任务映射到 User Story 1，便于追踪
- 每个任务完成后建议提交一次 commit
- 在 Phase 2 末尾即可用 Rust 测试验证 HDC 命令执行
- Tauri 默认在 macOS/Linux 上以 debug 模式运行时不限制 `std::process::Command`，无需额外权限配置
- 若遇到 CSP 限制导致前端无法正常渲染 Element Plus 样式，检查 `tauri.conf.json` 中 `app.security.csp` 配置
