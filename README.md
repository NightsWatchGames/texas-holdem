# Texas Holdem 德州扑克
- [x] 多房间支持
- [x] 旁观者支持
- [x] 游戏暂停恢复
- [ ] 发牌动画
- [ ] 翻牌动画
- [ ] 多人联机（C/S + 状态同步）
- [ ] 中途加入对局支持
- [ ] 断线重连
- [ ] AI托管
- [ ] 数据持久化
- [ ] 多server支持
- [ ] 游戏UI
- [ ] WASM支持

## 运行
本地
```
cargo run --bin texas-holdem-server
cargo run --bin texas-holdem-client
```

## 参考
- https://docs.unity3d.com/cn/2021.1/Manual/UNetOverview.html
- https://docs.unrealengine.com/4.27/zh-CN/InteractiveExperiences/Networking/QuickStart/
- https://github.com/dmackdev/bevy_jaipur
- [Wikipedia](https://en.wikipedia.org/wiki/Texas_hold_%27em)
- https://github.com/hmeine/zing-rs/tree/main

## 问题
**1.P2P和C/S网络架构理解？**
- https://zhuanlan.zhihu.com/p/56923109

**2.网络游戏数据同步方式？**