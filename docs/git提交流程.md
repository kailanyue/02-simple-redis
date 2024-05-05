
## Windows 中换行符解决
### 全局配置
```sh
git config --global core.autocrlf true

```

### 本地项目配置
```sh
git config --local core.autocrlf true
```

## git 提交流程
```sh
# 安装 pre-commit
pre-commit install

# 修改添加到暂存区
git add filename
git add .

# cargo deny 检查
cargo deny check -d

# 提交
git commit -m "提交内容"
```

## tag
```sh
# 更新 CHANGELOG
git-cliff -o CHANGELOG.md

# 添加 tag
git tag -a v1.0
git push -u origin v1.0

# 删除 tag
git tag -d v1.0
git push -d origin v1.7-csv
```
## 提交消息前缀
在 Git 中，有一些常见的提交消息前缀，可以帮助你更好地描述你的提交。以下是一些常见的前缀：

1. **feat**: 用于引入新功能。例如：`feat: 添加用户登录功能`
2. **fix**: 用于修复 bug。例如：`fix: 修复登录页面样式问题`
3. **chore**: 用于一般性的任务、构建或维护工作。例如：`chore: 更新依赖`
4. **docs**: 用于文档更新。例如：`docs: 更新 README`
5. **style**: 用于代码样式、格式化等。例如：`style: 调整缩进`
6. **refactor**: 用于重构代码，但不涉及功能更改。例如：`refactor: 优化数据处理逻辑`
7. **perf**: 用于性能优化。例如：`perf: 优化数据库查询`
8. **test**: 用于测试相关的提交。例如：`test: 添加用户登录测试用例`
9. **build**: 用于构建系统或外部依赖的更改。例如：`build: 更新 webpack 配置`
10. **ci**: 用于持续集成配置的更改。例如：`ci: 更新 GitHub Actions`


```sh
git commit -m "feat: Add new feature XYZ"
git commit -m "fix: Fix issue ABC"
```
