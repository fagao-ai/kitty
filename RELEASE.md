# 发布流程

## 版本管理

项目使用 `pnpm version` 命令来管理版本号，它会自动同步更新以下文件：
- `package.json` 的 `version` 字段
- `src-tauri/Cargo.toml` 的 `version` 字段

## 发布新版本

### 1. 更新版本号

```bash
# 补丁版本 (1.0.0 -> 1.0.1)
pnpm version patch

# 小版本 (1.0.0 -> 1.1.0)
pnpm version minor

# 大版本 (1.0.0 -> 2.0.0)
pnpm version major

# 或指定具体版本
pnpm version 1.2.3
```

### 2. 提交并推送

```bash
# 提交更改
git add .
git commit -m "bump version to x.x.x"

# 创建并推送 tag
git tag v1.0.0
git push origin main
git push origin v1.0.0
```

### 3. 自动构建

推送 tag 后，GitHub Actions 会自动：
1. 检出版本号
2. 构建 macOS (x86_64 + arm64)、Windows、Linux 版本
3. 创建 GitHub Release
4. 上传安装包

## 注意事项

- 版本号必须遵循语义化版本规范 (Semantic Versioning): `MAJOR.MINOR.PATCH`
- Tag 名称格式: `v` + 版本号，例如 `v1.0.0`
- UI 中显示的版本会自动从 `Cargo.toml` 读取，无需手动修改
