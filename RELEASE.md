# 发布流程

## 版本管理

项目使用 `pnpm version` 命令来管理版本号，它会自动：
- 更新 `package.json` 的 `version` 字段
- 同步更新 `src-tauri/Cargo.toml` 的 `version` 字段
- 创建 git commit
- 创建带 `v` 前缀的 git tag (如 `v1.0.0`)

## 发布新版本

### 1. 更新版本号

```bash
# 补丁版本 (1.0.0 -> 1.0.1) - bug 修复
pnpm version patch

# 小版本 (1.0.0 -> 1.1.0) - 新功能
pnpm version minor

# 大版本 (1.0.0 -> 2.0.0) - 破坏性变更
pnpm version major

# 或指定具体版本（必须高于当前版本）
pnpm version 1.2.3
```

### 2. 推送

```bash
# 推送 commit 和 tag
git push
git push --tags
```

### 3. 自动构建

推送 tag 后，GitHub Actions 会自动：
1. 检出版本号
2. 构建 macOS (x86_64 + arm64)、Windows、Linux 版本
3. 创建 GitHub Release
4. 上传安装包

## 注意事项

- 版本号必须遵循语义化版本规范 (Semantic Versioning): `MAJOR.MINOR.PATCH`
- `pnpm version` 需要在干净的工作目录中运行（无未提交的更改）
- Tag 会自动创建，格式为 `v` + 版本号，例如 `v1.0.0`
- UI 中显示的版本会自动从 `Cargo.toml` 读取，无需手动修改
- 不能重复设置相同的版本号（如当前是 1.0.0，不能再次执行 `pnpm version 1.0.0`）
