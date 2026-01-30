# 订阅管理功能设计文档

**日期**: 2026-01-30
**功能**: 订阅管理系统 (Subscription Management)
**状态**: 设计阶段

---

## 一、需求总结

### 核心功能
1. **管理订阅URL** - 支持添加、编辑、删除多个订阅源
2. **快速切换订阅** - 切换订阅时，清空旧订阅的节点，导入新订阅的节点
3. **刷新订阅** - 更新当前激活订阅的节点列表
4. **区分节点类型** - 手动添加的节点不受订阅切换影响，永久保存

### 节点生命周期
- **手动添加的节点**: `subscribe_id = NULL`，不会因订阅操作被删除
- **订阅导入的节点**: `subscribe_id != NULL`，跟随订阅切换或删除而变化
- **激活订阅**: 同时只有一个订阅的 `is_active = true`

---

## 二、数据库设计

### Subscribe表结构变更

**当前字段**：
```
- id: i32 (主键)
- url: String (订阅地址)
```

**新增字段**：
```
- name: String (订阅名称/备注)
- is_active: Boolean (是否为当前激活订阅，默认false)
- created_at: DateTime (首次添加时间)
- updated_at: DateTime (最后修改时间)
- last_sync_at: Option<DateTime> (最后同步成功时间)
```

**约束**：
- 同时只有一个 `is_active = true`（业务逻辑保证）
- 为 `is_active` 添加索引

### Xray表关联
- `subscribe_id = NULL` → 手动添加的节点
- `subscribe_id != NULL` → 订阅导入的节点，删除订阅时级联删除

---

## 三、后端API设计

### Tauri Commands

```rust
// 获取所有订阅列表（含节点统计）
#[tauri::command]
async fn get_all_subscriptions() -> Result<Vec<SubscriptionInfo>, String>

// 创建新订阅（仅保存信息，不导入节点）
#[tauri::command]
async fn create_subscription(name: String, url: String) -> Result<Subscribe, String>

// 更新订阅信息（名称和URL）
#[tauri::command]
async fn update_subscription(id: i32, name: String, url: String) -> Result<Subscribe, String>

// 删除订阅及其所有关联节点
#[tauri::command]
async fn delete_subscription(id: i32) -> Result<(), String>

// 切换订阅（清除旧节点，导入新节点）
#[tauri::command]
async fn switch_subscription(id: i32) -> Result<(), String>

// 刷新激活订阅的节点
#[tauri::command]
async fn refresh_subscription(id: i32) -> Result<(), String>
```

### 返回数据结构

```typescript
interface SubscriptionInfo {
  id: number
  name: string
  url: string
  isActive: boolean
  nodeCount: number
  createdAt: string
  updatedAt: string
  lastSyncAt?: string
}
```

---

## 四、前端架构

### 路由配置
```typescript
{
  path: '/subscription',
  name: 'subscription',
  component: () => import('@/views/subscription/SubscriptionView.vue')
}
```

### 菜单位置
在 `MenuView.vue` 中，位置：代理 → **订阅** ← 规则

### 文件结构
```
src/views/subscription/
  ├── SubscriptionView.vue         (主页面)
  ├── modal/
  │   ├── AddSubscription.vue      (添加订阅弹窗)
  │   └── EditSubscription.vue     (编辑订阅弹窗)
  └── store.ts                      (订阅状态管理)
```

---

## 五、UI设计

### SubscriptionView主页面

**布局**：
- HeaderBar: 标题 "订阅管理" + "添加订阅"按钮
- 卡片列表展示所有订阅
- 每个订阅卡片包含：名称、URL、节点数、最后更新时间

**订阅卡片样式**：
```
激活状态:
  🟢 机场A (激活中) ← 绿色圆点标记
  https://example.com/sub1
  节点数: 25 | 更新: 2026-01-30
  [切换] [刷新] [编辑] [删除]

非激活状态:
  ⚪ 备用线路
  https://example.com/sub2
  节点数: 18 | 更新: 2026-01-28
  [切换] [刷新] [编辑] [删除]
```

**操作按钮**：
- 切换：仅在非激活订阅上显示
- 刷新：仅在激活订阅上显示
- 编辑：修改名称和URL
- 删除：带二次确认弹窗

**NaiveUI组件**：
- `n-card` - 订阅卡片
- `n-badge` - 激活状态标记
- `n-button` - 操作按钮
- `n-popconfirm` - 删除确认
- `n-modal` - 添加/编辑弹窗

---

## 六、核心交互流程

### 1. 添加订阅
```
用户点击"添加订阅"
  ↓
AddSubscription弹窗 (输入名称、URL)
  ↓
create_subscription API
  ↓
[可选] 立即切换到此订阅 → switch_subscription
  ↓
刷新列表显示
```

### 2. 切换订阅
```
用户点击"切换"按钮
  ↓
确认弹窗: "将删除当前订阅的X个节点"
  ↓
switch_subscription API:
  - 删除旧激活订阅的所有节点 (WHERE subscribe_id = old_id)
  - 设置新订阅 is_active = true
  - 拉取并导入新订阅的节点
  - 更新 last_sync_at
  - 清除激活代理状态 (如果被删除)
  ↓
刷新列表显示
```

### 3. 刷新订阅
```
用户点击"刷新"按钮 (仅激活订阅)
  ↓
refresh_subscription API:
  - 删除该订阅的旧节点
  - 重新拉取并导入
  - 更新 last_sync_at
  ↓
成功消息: "已刷新，导入了X个节点"
```

### 4. 删除订阅
```
用户点击"删除"按钮
  ↓
Popconfirm: "删除会同时删除X个节点"
  ↓
delete_subscription API:
  - 级联删除所有关联节点
  - 清除激活状态 (如果删除的是激活订阅)
  ↓
刷新列表显示
```

---

## 七、实现注意事项

### 数据一致性
- 切换订阅时，需要原子性操作（事务）
- 删除激活订阅的节点后，需要清除 ProxyStore 中的激活代理ID

### 错误处理
- 网络错误导致导入失败时，需要回滚订阅状态
- 订阅URL无效时，给出清晰的错误提示

### 用户体验
- 切换/刷新/删除时显示加载状态
- 大量节点导入时，显示进度反馈
- 操作完成后显示成功/失败消息

### 复用现有代码
- 节点导入逻辑复用 `parse_subscription`
- 导入时设置 `subscribe_id` 字段
- 代理页面的节点列表默认过滤显示 (激活订阅节点 + 手动节点)

---

## 八、成功指标

✅ 用户可以管理多个订阅URL
✅ 快速切换订阅，旧节点自动清除
✅ 手动添加的节点不受订阅影响
✅ 可以刷新单个订阅获取最新节点
✅ 删除订阅时清理关联数据
✅ UI清晰直观，操作简单易用

