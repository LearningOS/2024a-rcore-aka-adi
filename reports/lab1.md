## 实现

1，在TCB中加入TaskInfo

2，trap时记录syscall次数

3，记录task第一次运行时间

4，完成task_info syscall，返回TaskInfo

  

## 简答

### 1,

使用的sbi
```
[rustsbi] RustSBI version 0.3.0-alpha.2
```

程序尝试写入0x0地址。
```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
```

程序尝试在U态调用sret。 sret为特权指令。
```
[kernel] IllegalInstruction in application, kernel killed it.
```

程序尝试在U态读取sstatus。sstatus为特权寄存器。
```
[kernel] IllegalInstruction in application, kernel killed it.
```

  

### 2，
1，a0为内核的栈指针, \_\_restore使用场景
```
1, 从中断返回用户态程序
2，构造特殊trap上下文，用作切换程序
```

2，
```
sstatus CPU的特权级，代表返回后的特权级
sepc 指令的地址，代表返回后的将执行的指令地址；
sscratch 存储的是返回后的栈指针。
```
3，
```
跳过x2是可以在后面再保存
跳过x4是因为暂时不需要使用x4
```
4，
```
sp指向用户栈，sscratch指向内核栈
```
5，
```
sret，该指令为特权指令，由S态返回U态
```
6，
```
sp指向内核栈，sscratch指向用户栈
```
7，
```
在trap处理前由硬件自动完成
```

## 荣誉守则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
    
    无
    
2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
    
    >[rCore-Camp-Guide-2024A 文档](https://learningos.cn/rCore-Camp-Guide-2024A/index.html)
    

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。