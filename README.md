# yapi2js

**极端不稳定，请勿生产环境使用。**

yapi 导出 json 配置。 生成 ts request 和 response type， 并配合模版生成 api.ts。

```bash
yapi2js --help
```
```
yapi 0.1

USAGE:
    yapi --in <IN> --out-file <OUT_FILE>

OPTIONS:
    -h, --help                   Print help information
    -i, --in <IN>                
    -o, --out-file <OUT_FILE>    
    -V, --version                Print version information

```

example:

```bash
yapi2js --in ./api.yapi --out-file ./api.ts
```

```bash
yapi2js --in https://... --out-file ./api.ts
```


DONE：
- [x] ts request type
- [x] config from yapi uri

Next TODO:
- [ ] 单元测试
- [ ] 代码规范
- [ ] 产出文件标准化
- [ ] mock 服务器
- [ ] 模版定制
- [ ] ...
