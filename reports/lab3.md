## 实现
1，实现spawn，实现方式仿照exec，读取到程序加入任务队列

2，实现调度算法，参照任务书所说实现算法，为了编码简单使用了遍历来确定任务优先级。

## 简答

### 1,
```
(1) 不是p1先执行，因为发生了溢出

(2) 易得，0次调度满足STRIDE_MAX – STRIDE_MIN <= BigStride / 2。
设k次调度以后的pass排列为t1*BigStride/p1,t2*BigStride/p2,..tx*BigStride/px。满足STRIDE_MAX – STRIDE_MIN <= BigStride / 2。
	k+1次调度后可能的排列有三种：
		t2*BigStride/p2,..tx*BigStride/px，(t1+1)*BigStride/p1
		(t1+1)*BigStride/p1,t2*BigStride/p2,..tx*BigStride/px
		t2*BigStride/p2,..(t1+1)*BigStride/p1..,tx*BigStride/px
	对后两种而言，必然满足STRIDE_MAX – STRIDE_MIN <= BigStride / 2。
	对第一种，STRIDE_MAX – STRIDE_MIN = (t1+1)*BigStride/p1 - t2*BigStride/p2 <=
	BigStride / 2 + t1*BigStride/p1 - t2*BigStride/p2 <= BigStride / 2。
	因此，满足STRIDE_MAX – STRIDE_MIN <= BigStride / 2。

(3) 
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	    if self.0 < other.0 && other.0 - self.0 <= BigStride / 2 {
		    Some(Ordering::Less)
	    }
	    else {
		    Some(Ordering::Greater)
	    }
    }
}
```


## 荣誉守则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
    
    无
    
2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
    
    >[rCore-Camp-Guide-2024A 文档](https://learningos.cn/rCore-Camp-Guide-2024A/index.html)
    

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。