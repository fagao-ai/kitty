const fs = require('node:fs')
const path = require('node:path')

const packageJsonPath = path.join(__dirname, 'package.json')

// 读取package.json文件
fs.readFile(packageJsonPath, 'utf8', (err, data) => {
  if (err) {
    // console.error('Error reading package.json:', err);
    return
  }

  try {
    const packageJson = JSON.parse(data)

    // 假设你想手动设置新的版本号
    const newVersion = '1.0.1'

    // 更新版本号
    packageJson.version = newVersion

    // 将更新后的内容写回package.json文件
    fs.writeFile(packageJsonPath, JSON.stringify(packageJson, null, 2), 'utf8', (err) => {
      if (err) {
        // console.error('Error writing package.json:', err);
      }
      else {
        // console.log(`package.json version updated to ${newVersion}`);
      }
    })
  }
  catch (parseErr) {
    // console.error('Error parsing package.json:', parseErr);
  }
})
