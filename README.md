# Test write fb

能够在DragonOS中，使用`/dev/fb0`进行绘图，
绘制屏幕四角的颜色块，以及中间的DragonOS Logo。
并截图保存。

## 使用

在编译DragonOS时，加入当前程序的dadk配置文件即可编译进去。

在DragonOS启动后，使用`test-write-fb`即可运行。