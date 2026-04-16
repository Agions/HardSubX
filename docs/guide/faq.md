# 常见问题

## 安装与运行

### pnpm install 失败

```bash
# 清除缓存重试
pnpm store prune
rm -rf node_modules pnpm-lock.yaml
pnpm install
```

### Rust 编译失败

```bash
# 确保 Rust 最新
rustup update
rustc --version  # >= 1.70
```

### 提示 "WebView2 not found"（Windows）

下载安装 [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)。

### 提示 "webkit2gtk not found"（Linux）

```bash
sudo apt-get install libwebkit2gtk-4.1-dev
```

---

## OCR 识别

### 字幕识别不准确

1. **调整 ROI**：确保只框住字幕区域，排除水印/台标
2. **降低置信度阈值**：尝试 60% 而非默认 70%
3. **启用 Multi-pass OCR**：同一区域识别多次，取最优结果
4. **切换引擎**：EasyOCR 对自然场景字幕效果更好

### 中文识别错误

- 启用 **Text post-processing**（文本后处理）进行繁简转换和纠错
- 确认 ROI 内无 Logo 干扰

### 字幕被截断或合并

- 启用 **Subtitle merge**（字幕合并），调整相似度阈值到 80%
- 手动在导出后编辑 JSON 修正

### OCR 速度很慢

- 使用 **Tesseract.js**（WASM，无需额外依赖，最快）
- 启用 **Scene detection**（场景检测）跳过无字幕帧
- NVIDIA GPU 用户：切换到 PaddleOCR + GPU 加速

---

## 视频格式

### 不支持的格式

SubLens 依赖 FFmpeg 进行解码。常见支持格式：

✅ **MP4** · **MKV** · **AVI** · **MOV** · **WebM** ✅ **M4V** · **WMV** · **FLV** · **3GP**

❌ 不支持：DRM 加密视频（Netflix、Disney+ 等）

### Variable Frame Rate (VFR) 视频

SubLens 自动处理 VFR 视频，确保时间戳准确。

---

## 导出格式

### SRT 时间轴错位

导出时取消 **Frame-mapped** 模式，或改用 **JSON** 格式（包含帧映射）。

### ASS/SSA 样式丢失

ASS/SSA 支持高级样式（字体/颜色/位置），但部分播放器不支持所有特性。WebVTT 是更好的兼容性选择。

---

## 其他

### 如何报告 Bug？

请在 [GitHub Issues](https://github.com/Agions/SubLens/issues) 创建 issue，包含：
- 操作系统和版本
- SubLens 版本
- 复现步骤
- 相关视频截图/日志

### 是否支持批量处理？

是的，在 **Progress** 标签页可添加多文件到队列。或使用 CLI：

```bash
for f in *.mp4; do sublens-cli extract "$f" --output "./out/$f"; done
```
