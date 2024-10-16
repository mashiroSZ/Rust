# Redis

---

---

---

记录Redis笔记

依旧是使用四级标题记录**笔记糖**



## 基础

### redis是什么

remote dictionary server 远程字典服务，使用ANSIC遵循BSD编写的Key-value数据库

服务于内存当中 完全开源



### redis能干什么

就像是挡在 MySQL 前面的带刀护卫

中间的一道墙，由于80%select 20%alert 因此提高性能，减负，redis分担查询服务，mysql用作储存，在硬盘当中

而redis的快在于：

- redis在内存当中，mysql在硬盘
- redis的数据结构是kv键值对
- RDBMS，NOSQL概念，是一个键值对而不是传统的关系型数据库

![](.\img_store\屏幕截图 2024-10-07 204919.png)

有事先找redis 

- 读取速度一秒11，0000次 写入80000次每秒
- redis数据结构也不止是kv键值对，list,set,zset,hash等数据结构
- Redis支持数据持久化，在内存中工作是也会同时将数据备份在相应硬盘当中
- Redis支持数据备份，即master-slave模式的数据备份(就是主从备份)

Redis集群 高可用性

<img src=".\img_store\屏幕截图 2024-10-07 205614.png" style="zoom:33%;" />



### 下载安装

redis目前不支持windows，因此我是用linux/Ubuntu服务器进行操作

依旧是使用datagrip作为远程客户端连接服务器redis服务器

修改一下密码然后改一下bind

``` bash
bind= 0.0.0.0 ::
#这个：：是用于ipv6的回环地址
```



## 核心特性介绍

- [十大数据结构](#Base Data Structures 基本数据结构)
- [持久化](#持久化)
- [事务](#事务)
- [发布订阅](#发布订阅模式)
- [Lua 脚本](#Lua脚本)
- [高可用性与分区](#高可用性与分区)
- [持久性与备份](#持久性与备份)
- [安全性](#安全性)
- [命令与优化](#命令与优化)
- [内存管理](#内存管理)
- [Redis 客户端与连接池](#redis-客户端与连接池)
- [监控与运维工具](#监控与运维工具)



## Base Data Structures 基本数据结构

十大基本数据结构

|    类型     |   含义   |
| :---------: | :------: |
|   String    |  字符串  |
|    hash     |   哈希   |
|    list     |   列表   |
|     set     |   集合   |
|    Zset     | 有序集合 |
|     GEO     | 地理空间 |
| HyperLogLog | 基数统计 |
|   bitmap    |   位图   |
|  bitfield   |   位域   |
|   Stream    |    流    |

之后的分类我就不一个一个的写常用命令了，大多是一样的

发现没有必要，也许在常用命令可以逐步的加

---

### String

即是字符串

经典的k-v键值对

**常用命令**

- set key value 设置数据
- get key 获取数据
- ttl key 剩余时间



属性

![](C:\Users\70337\Desktop\LM\Redis_study\img_store\屏幕截图 2024-10-09 180822.png)

这里有什么存在时间过期时间等，反正到时候要用ml肯定是借助gpt的



#### 笔记糖 获取unix时间戳

在java中，可以使用

 ``` java
 System.out.println(Long.toString(System.currentTimeMills()/1000L));
 ```

获取当前unix系统时间戳



---

### list

表现为 k v1 v2...

底层为经典的双端列表



**常用命令**

- LPUSH RPUSH

插入元素，L左插入，R右插入

- LPOP RPOP

弹出元素

- LLEN

获取列表长度

- LRANGE

根据下标获取列表一个范围内的长度

``` bash
LRANGE Key 0 -1 
#全部元素 下标支持使用负值从右边开始
LRANGE Key 0 2
```

- LINDEX

获取指定下标的元素值

- LSET

设置指定下标元素

- LREM

移除列表与给定值值相等的元素

- LINSERT

在列表指定元素之前或者之后插入一个元素

``` bash
LINSERT mylist BEFORE "111" "222"
```

- LTRIM

裁剪列表，根据下标

- RPOPLPUSH

从第一个列表的尾部弹出元素到第二个列表的头部

``` bash
RPOPLPUSH firstlist secondlist
```

- BLPOP BRPOP


阻塞的弹出元素

---



### hash

很像json数据格式

``` bash
user:1001  {
    "name": "Alice",
    "age": "30",
    "email": "alice@example.com"
}
```

一个大k名，多个kv键值对

**常用命令**

- HSET （取代了HMSET）

设置hash值

``` bash
HSET key field value
```

- HGET

获取哈希值

- HMGET

获取一个或者多个哈希值

- HGETALL

获取全部hash值

- HDEL

删除一个或多个字段

- HLEN

获取哈希的长度

- HEXISTS

检测某个哈希是否存在

- HKEYS

获取某个哈希的全部字段值

- HVALS

获取全部属性值

- HINCRBY

给哈希某个字段加上整数值

- HINCRBYFLOAT

浮点数值



---

### set

数据结构

一个k多个values

 ``` bash
 fruits -> {"apple", "banana", "orange"}
 ```



**常用命令**

- SADD 

向一个集合中添加元素

- SREM

从一个集合中移除一个或多个元素

- SMEMBERS

获取集合中所有元素

- SISMEMBER

确认集合中是否包含指定元素

``` bash
SISMEMBER fruits "apple"
```

- SCARD

返回集合中的数量

- SRANDMEMBER

随机返回元素中一个或多个元素

- SPOP

随即弹出数个元素

- SMOVE

将一个元素移动到另一个集合



**集合运算**

- SINTER

返回指定集合的交集

- SUNION

并集

- SDIFF

返回指定集合的差集

- SINTERSTORE

获取并集并且储存在新的集合当中

``` bash
SINTERSTORE common fruits tropical
SMEMBERS common
```

- SSCAN

增量的迭代集合中的元素，适用于大集合避免阻塞



---

家人们懒得敲了上4o~！

### Zset

数据结构

一个键对应多个成员，每个成员关联一个分数（score），根据分数有序排列。

```bash
leaderboard -> {"Alice": 100, "Bob": 90, "Charlie": 80}
```

**常用命令**

- ZADD

向有序集合中添加元素并设置其分数。

```bash
ZADD leaderboard 100 "Alice" 90 "Bob" 80 "Charlie"
```

- ZREM

从有序集合中移除一个或多个元素。

```bash
ZREM leaderboard "Charlie"
```

- ZSCORE

返回指定元素的分数。

```bash
ZSCORE leaderboard "Alice"
```

- ZRANK

返回指定成员的排名（按分数从低到高）。

```bash
ZRANK leaderboard "Bob"
```

- ZREVRANK

返回指定成员的排名（按分数从高到低）。

```bash
ZREVRANK leaderboard "Bob"
```

- ZRANGE

按分数排序，返回指定区间内的元素（从低到高）。

```bash
ZRANGE leaderboard 0 -1
```

- ZREVRANGE

按分数排序，返回指定区间内的元素（从高到低）。

```bash
ZREVRANGE leaderboard 0 -1
```

- ZCARD

返回有序集合中元素的数量。

```bash
ZCARD leaderboard
```

- ZINCRBY

为指定成员的分数增加给定值。

```bash
ZINCRBY leaderboard 10 "Alice"
```

- ZRANGEBYSCORE

返回指定分数范围内的成员。

```bash
ZRANGEBYSCORE leaderboard 80 100
```

- ZREMRangeByScore

移除指定分数范围内的所有成员。

```bash
ZREMRANGEBYSCORE leaderboard 0 50
```

- ZREMRangeByRank

移除指定排名范围内的所有成员。

```bash
ZREMRANGEBYRANK leaderboard 0 1
```

**集合运算**

- ZINTERSTORE

计算给定有序集合的交集，并将结果存储在新的集合中。

```bash
ZINTERSTORE result 2 set1 set2
```

- ZUNIONSTORE

计算给定有序集合的并集，并将结果存储在新的集合中。

```bash
ZUNIONSTORE result 2 set1 set2
```

- ZSCAN

增量地迭代有序集合中的元素，适用于大集合以避免阻塞。

```bash
ZSCAN leaderboard 0 MATCH "A*"
```



---

暂时不搞剩下五个先往后施工







## 持久化



### 基础介绍

**是什么**

在指定的时间间隔 执行数据集的快照

在指定时间间隔内将内存中的数据集快照写入磁盘，执行Snapshot内存快照，恢复时将硬盘快照文件读回内存里

**能干啥**

一锅端

redis的数据都在内存当中，保存备份时执行的是 **全量快照** 把内存当中的所有数据都记录到磁盘当中一锅端

Rdb保存的是dump.rdb文件





使用 Redis 的持久化功能非常简单，只需要在配置文件中进行一些设置，或者通过命令行进行配置。以下是如何设置和使用 Redis 的 RDB 和 AOF 持久化的详细步骤：

### 1. RDB 持久化设置

#### 配置 RDB

- **编辑配置文件**：找到 Redis 的配置文件（通常是 `redis.conf`），使用文本编辑器打开它。

- **设置保存条件**：在配置文件中，找到以下行（通常是以 `save` 开头）：

  ```plaintext
  save 900 1     # 如果900秒内至少有1个键被修改
  save 300 10    # 如果300秒内至少有10个键被修改
  save 60 10000  # 如果60秒内至少有10000个键被修改
  ```

  可以根据需要修改这些参数，表示在指定的时间内满足特定条件后生成 RDB 快照。

- **指定 RDB 文件位置**：你还可以设置 RDB 文件的存储位置（默认是 `/var/lib/redis/dump.rdb`）：

  ```plaintext
  dir /var/lib/redis/
  ```

#### 手动生成 RDB 快照

- 使用命令：
  - `SAVE`：阻塞当前客户端，直到生成快照完成。
  - `BGSAVE`：在后台生成快照，不会阻塞当前客户端。

### 2. AOF 持久化设置

#### 配置 AOF

- **编辑配置文件**：同样在 `redis.conf` 中，找到以下行：

  ```plaintext
  appendonly no
  ```

  将 `no` 改为 `yes` 以启用 AOF 持久化：

  ```plaintext
  appendonly yes
  ```

- **设置 AOF 文件位置**：可以设置 AOF 文件的存储位置（默认是 `/var/lib/redis/appendonly.aof`）：

  ```plaintext
  appendfilename "appendonly.aof"
  ```

- **配置同步策略**：在 `redis.conf` 中可以设置 AOF 的同步策略，例如：

  ```plaintext
  appendfsync always   # 每次写入都同步
  appendfsync everysec # 每秒同步一次
  appendfsync no       # 不进行同步
  ```

### 3. 启动 Redis 服务

在配置完成后，重新启动 Redis 服务以使更改生效：

```bash
redis-server /path/to/redis.conf
```

### 4. 数据持久化的验证

- 在 Redis 启动后，你可以使用以下命令验证持久化是否生效：

  - **检查 RDB 文件**：确认 RDB 文件是否在指定目录中。

  - **检查 AOF 文件**：确认 AOF 文件是否在指定目录中。

- 在 Redis 中插入一些数据，然后重启 Redis 服务，检查数据是否能成功恢复。

### 5. 恢复数据

- Redis 在启动时会自动检查是否存在 RDB 或 AOF 文件并尝试从中恢复数据。如果两个文件都存在，默认使用 AOF 文件。



前面没有提到具体在cli中的操作 我一起给出以后应该看的懂

``` bash
CONFIG SET save "900 1" "300 10" "60 100"
#每 900/300/60 发生1/10/100即发生快照 储存进入rdb文件当中
SAVE / BGSAVE
#手动快照 阻塞 / 异步
#----#
CONFIG SET appendonly yes
#启用追加文件 AOF 
CONFIG SET appendfsync everysec
#配置AOF策略 就是上边同步策略 平衡数据安全性与性能
```



---

## 事务

如同MySQL事务，一组顺序操作的命令

- 作为一个单独的操作单元
- 不会被其他命令打断
- 保持数据一致性
- 全成功/全失败

**基本命令**

- MULTI 开启事务
- EXEC 执行事务内所有命令
- DISCARD 取消事务
- WATCH 监视多个键，如果有变动，则中断
- UNWATCH 取消监视

#### 笔记糖 相对于mysql的事务

mysql具有AICD atomicity consistency isolution durability 

- start transaction 
- commit
- rollback
- savepoint
- release savepoint 

至于使用在后面练习用gpt出题带答案，非常好用

---

## 发布订阅模式

Pub/Sub

publisher subscriber

~  这个在vue学习中的消息传递中有提到这个订阅与发布 逻辑类似  ~

**要素**：

1. Channel 频道 发布者将消息发布到频道
2. Subscriber 订阅者 监听频道
3. 传递 redis维护 订阅者列表 发布者发布到频道然后redis遍历列表发布到所有订阅了此频道的订阅者

cli：

- SUBSCRIBE channel1 channel2
- PUBLISH channel1 message
- UNSUBSCRIBE channel

客户端与redis系统

<img src="./img_store\进程关系.png" alt="进程关系" style="zoom:33%;" />

可以看到，如果只有一个redis系统，服务器中的不同进程使用的redis服务是共享内存的，他们之间使用同一redis系统

因此就算不同进程之间拥有一定的数据独立性，仍然会有数据干扰的风险，于是我比较欣赏下面两种方案：

1. 进程单独部署redis

使用docker为对应的进程单独部署redis系统，拥有独立的内存空间/数据库/端口

2. 使用redis集群/框架

这个不清楚以后学

#### 笔记糖 关于docker

很多地方都出现docker的身影，是在有必要了解

**容器** 是docker的核心概念，旨在使用容器进行打包，部署，移植开发环境

提高可移植性与可管理性，非常的轻量级，形成的包镜像可以在任何支持docker的环境部署

DOCKER FILE 文本文件 构建docker镜像的所有配置参数 

DOCEKR HUB 共享 发布 中心

#### 笔记糖 CICD

持续继承/持续部署这个也得学

---

## Lua脚本

**是什么**

可以在自定义code，将其封装进入脚本实现高效率复现复杂操作（我是不太清楚和事务的区别的）

实现**原子性**操作，减少网络延迟和提高执行效率

**角色与重要性**

Lua脚本是一种服务器拓展机制，提供了复杂操作的简易请求方式，降低网络请求压力



**实现机制**

redis将lua脚本放在内存中进行编译，阻塞其他命令的进行，确保原子性和一致性



**Cli**

``` bash
EVAL "return ARGV[1]" 0 "Hello, Redis!"
#执行命令 Lua脚本内容 传入键的数量 传入参数
EVAL <script> <numkeys> <key1> <key2> ... <keyN> <arg1> <arg2> ... <argM>
#多个参数顺序对应
```



---

## 高可用性与分区

**是什么**

高可用性与分区 是redis处理大规模**分布式**数据的一种机制。高可用性使reids实例在发生故障时仍然可用，分区/分片将数据分布到多个节点，提高性能及可扩展性

### 高可用性

**哨兵/集群/主从复制**都是这一块的内容

**模块与角色**

- 主从复制 redis主从架构当中 主节点负责写入 从节点负责读取和数据备份
- reids sentinel 哨兵 用于监控Redis主从复制系统 实现自动故障转移 确保高可用性
- Redis Cluster reids集群 提供分区功能，将数据均匀分布在多个节点上，并且允许节点之间相互调节

**主从复制** 将一个redis实例设置为主节点，其他实例作为从节点，从节点会实时复制主节点的数据，保证主节点宕机，从节点能顶上去，

主节点发送增量数据给从节点，从节点通过同步主节点的数据变化

**哨兵模式** 监视主从架构中的主节点，若主节点故障，则自动切换为从节点。监视主节点依靠心跳机制，若主节点故障，则自动切换为从节点，实现自动故障转移，避免服务中断，

依赖心跳检测/选举机制，通过raft类算法选择新的主节点

**集群** 在主从架构的基础上，拓展分片储存和查询，使得系统具备更强的拓展性，高可用

基于哈希算法实现数据分布，并且采用gossip协议进行节点间的健康状态检测



#### 笔记糖 守护进程启动

``` bash
redis-server --port 6380 --daemonize yes
```

应用放置于后台运行，而不必受到前台调度影响



**cli**

如何创建新节点？如何切换节点主次状态？如何手动故障节点观察节点切换机制？

#### **启动实例**

``` bash
redis-server --port 6380
#新实例作用于6380
redis-server --port 6380 --daemonize yes
#守护进程式启动（后台模式）
```

**警告内存过度提交**

``` bash
sudo sysctl vm.overcommit_memory=1
#临时修改系统，启动内存过度提交
```

就是内存不够可能会影响redis的持久化RDB/AOF警告



现在就拥有两个redis实例，如何确认redis-cli连接以及查看运行状态？

redis-cli**默认连接6379端口**redis实例

``` bash
redis-cli -p 6380
#特定端口连接
#类似mysql -u root -p，由于redis默认无用户名
redis-cli -h <IP地址> -p <端口>
#远程连接
```

状态检测

由于systemctl状态管理服务只能监听systemd启动的服务，但是大多数情况下，Redis实例通过redis服务进行注册，systemctl status redis-server 只能检测这一个编写过systemd文件的服务

于是通过通用方式进行检测

``` bash
redis-cli -p 6380 INFO
#返回 Redis 实例的详细状态信息，包括服务器信息、内存使用情况、客户端连接数、CPU 使用率等。
redis-cli -p 6380 PING
#确认实例是否正常运行，如果是，返回PONG
```

**关闭实例**

如果是前台，直接ctrl c

守护进程：

``` bash
redis-cli -p 6380 shutdown
#在对应的cli中直接使用shutdown

#如果需要密码
redis-cli -p 6380 -a your_password shutdown
```

如果无法通过redis命令行关闭实例，那么直接使用操作系统关闭实例进程

``` bash
#用超级权限直接杀死进程
sudo pkill -f "redis-server.*6380"

#找到运行在6380端口的redi实例pid
ps aux | grep redis-server
#返回redis   12345  0.1  0.5 127456  23412 ?  Ssl  18:10   0:05 redis-server 127.0.0.1:6380

#关闭
sudo kill 12345
#强行终止
sudo kill -9 12345
```

可以进行上边的连接试试是否还在运行，未运行则返回错误信息

``` bash
#Could not connect to Redis at 127.0.0.1:6380: Connection refused
```

#### **配置从节点**

``` bash
#比如我有两个redis实例，分别是6380/6379端口
redis-cli -p 6380 SLAVEOF 127.0.0.1 6379
#可以使用命令查看主从架构状态
redis-cli -p 6379 INFO replication
#个逼样的，查看架构状态还要身份验证
root@lavm-ifal2r1mgt:~# redis-cli -p 6379 -a Redis-GongJianHao INFO replication
Warning: Using a password with '-a' or '-u' option on the command line interface may not be safe.
# Replication 
role:master
connected_slaves:0
master_failover_state:no-failover
master_replid:84cc6792c3255961b57691c10da5dad1bdf0dddf
master_replid2:0000000000000000000000000000000000000000
master_repl_offset:0
second_repl_offset:-1
repl_backlog_active:0
repl_backlog_size:1048576
repl_backlog_first_byte_offset:0
repl_backlog_histlen:0
root@lavm-ifal2r1mgt:~# 
#反正失败的大概就是这样 因为要保证6379 6380同时在运行状态

```

然后进行检测，看看怎么工作的 

主节点设置数据， 从节点查看数据，然后手动故障主节点，看看从节点如何变成主节点的

妈的故障就是直接停止于是我尝试了生产环境中遇到的常见问题

- 网络故障
- cpu负载满
- 服务器宕机
- 硬盘损坏

但是我的从节点于主节点都是在一个服务器上，因此不尝试XD

### 哨兵sentinel /集群cluster

至于主从架构发生错误，只会断开主从节点的连接，并不会自动切换，需要进行配置哨兵/cluster才能达到监视主从架构运行状态并且在主节点发生故障之后自动切换到从节点进行工作

这俩的功能我觉得类似，以下是区别：



Redis Sentinel 和 Redis Cluster 都是 Redis 提供的高可用和扩展解决方案，但它们的功能和用途有所不同。我们可以通过以下几个关键点来比较它们的运行功能以及应用场景：

#### 1. **基本功能对比**

| 功能           | **Redis Sentinel**                                           | **Redis Cluster**                                            |
| -------------- | ------------------------------------------------------------ | ------------------------------------------------------------ |
| **高可用性**   | 提供高可用性，监控主从节点并在主节点故障时执行自动故障切换（failover） | 提供高可用性，通过将数据分布到多个节点，支持自动故障切换     |
| **数据分布**   | 不支持数据分布。每个主节点都是全量持有数据，并通过主从复制方式保持备份 | 支持数据分片（sharding），每个节点存储部分数据，支持水平扩展 |
| **扩展性**     | 适合单主从结构，主要用于小规模实例的高可用管理               | 支持水平扩展，大规模数据集可以分布在多个节点上，支持数百个节点 |
| **一致性**     | 使用主从复制模型，主节点负责写入，从节点可以负责读取；切换后可能会有短暂的不可用 | 数据分片并且使用部分一致性，节点之间通过哈希槽来管理数据分布 |
| **故障切换**   | 支持自动故障切换，主节点故障时由哨兵选举一个从节点提升为主节点 | 节点故障时自动切换，重新分配数据槽到其他节点                 |
| **客户端操作** | 使用普通的 Redis 客户端，不需要特殊支持                      | 客户端需要支持 Redis Cluster 特有的指令，如 `MOVED` 重定向   |
| **写操作**     | 写操作只能发往主节点，支持读写分离（从节点处理读取操作）     | 写操作根据哈希槽路由到相应的节点，写入分布到不同的节点上     |
| **运维复杂度** | 比较简单，哨兵只负责监控和故障切换                           | 复杂度较高，尤其在管理和扩展节点时需要注意数据槽的管理       |

#### 2. **使用场景**

##### **Redis Sentinel**
Redis Sentinel 主要用于提供 **高可用性**，它可以在主节点故障时自动故障转移到从节点，以确保服务的持续运行。它的特点是：
- **主从复制**：Sentinel 是基于 Redis 的主从复制机制来实现的，每个主节点有多个从节点进行备份。
- **适合小型部署**：当你只需要较少的 Redis 实例时，Sentinel 非常适合。它不支持水平扩展（即数据分片），适用于数据量较小的场景。
- **易于运维**：Sentinel 的配置相对简单，可以在较小规模的应用中快速搭建高可用 Redis 系统。

##### **Redis Cluster**
Redis Cluster 主要用于同时提供 **高可用性** 和 **水平扩展**，它通过将数据分片存储在多个节点上，支持**大规模数据集**和**高并发**的应用场景。它的特点是：
- **数据分片**：Cluster 使用哈希槽（hash slots）将数据分布在不同的节点上，支持数据的水平扩展。
- **适合大规模应用**：当应用需要存储大量数据并且对高可用性有要求时，Cluster 是更好的选择。Cluster 还支持多个节点分摊写操作，提升写入性能。
- **自动重分配**：当节点故障时，Cluster 会自动将数据重分配到其他节点，保证服务的高可用性。

#### 3. **是否只使用其中一个？**
在现代 Redis 部署中，一般情况下，应用程序 **只会启用 Sentinel 或 Cluster 中的一个功能**，具体取决于应用的需求：

- **仅使用 Sentinel**：如果你的 Redis 集群规模较小（比如 1 个主节点和几个从节点），并且你不需要水平扩展，只需要高可用性，那么 Redis Sentinel 就足够了。Sentinel 配置简单，故障切换能力足以应对常见的主节点宕机场景。

- **仅使用 Redis Cluster**：如果你的应用需要处理海量数据和高并发，且需要数据分片、自动扩展等特性，那么使用 Redis Cluster 是更合适的选择。它不仅能够实现高可用性，还可以通过数据分片提高存储容量和处理能力。

#### 4. **可以结合使用吗？**
Redis Sentinel 和 Redis Cluster **不能** 直接结合使用。因为：
- Sentinel 是为 Redis 的主从复制设计的，不支持数据分片。
- Redis Cluster 是分布式存储系统，提供了内置的高可用机制，Sentinel 的功能在 Cluster 模式下是冗余的。

#### 5. **现代主流应用使用场景**
- **小型部署（数据量小，主要追求高可用性）**：使用 Redis Sentinel 搭配主从复制，简化部署和运维。
- **大规模部署（需要数据分片和水平扩展）**：使用 Redis Cluster 提供数据分片和高可用性，支持高并发和大数据量应用。

！！！

**~~操他妈的4o，别几把在linux文件里乱写注释了~~**



### 分区  大量数据

#### 分区（Sharding）是什么

分区（Sharding）是一种将数据分布到多个 Redis 实例上的方法。通过将数据拆分为多个部分，每个部分存储在不同的 Redis 实例上，分区可以提升系统的扩展性、并发处理能力以及数据存储容量。

#### 分区的原理

分区的核心思想是基于某种规则（如哈希或范围）将数据按键值划分。Redis 会根据键的某个特性决定将其存储在哪个 Redis 实例中。

**原理**：
1. Redis 通过一个分区算法（如一致性哈希算法）计算每个键的哈希值。
2. 将计算出来的哈希值分配到一个具体的 Redis 节点（实例）。
3. 客户端根据哈希结果将数据发送到对应的 Redis 实例。

#### 如何实现分区

在 Redis 中有两种常用的分区实现方式：

1. **客户端分区（Client-side Sharding）**：
   通过客户端来决定数据存储在哪个 Redis 实例。客户端代码中实现分区逻辑，常见于自行管理分区的场景。
   
2. **Redis Cluster 分区**：
   Redis 原生的集群模式，通过 Redis Cluster 自动管理分区、数据分布和故障恢复。Redis Cluster 将键空间分成 16,384 个槽（hash slots），并自动分配到不同的 Redis 实例。

#### 使用 Redis CLI 操作分区

Redis 集群环境下，可以使用 `redis-cli` 进行分区操作。以下是几个常用的操作步骤：

1. **查看节点状态**：
   使用命令连接 Redis 集群中的一个实例，查看当前集群信息。
   
   ```bash
   redis-cli -c -h <host> -p <port> cluster nodes
   
2. **手动迁移数据槽**：
   将某个槽从一个节点迁移到另一个节点。
   
   ```bash
   redis-cli -h <source-host> -p <source-port> cluster setslot <slot> migrating <destination-node-id>
   redis-cli -h <destination-host> -p <destination-port> cluster setslot <slot> importing <source-node-id>
   ```
   
3. **手动添加新节点**：
   将新的 Redis 实例加入集群，并分配数据槽。
   ```bash
   redis-cli --cluster add-node <new-host>:<new-port> <existing-host>:<existing-port>
   ```

4. **自动重新分配槽**：
   自动将数据槽在集群中重新平衡分布。
   ```bash
   redis-cli --cluster rebalance <existing-host>:<existing-port>
   ```

分区是一种提升 Redis 扩展性和性能的关键技术，主要通过将数据拆分到多个 Redis 实例来实现。在 Redis 中，分区可以通过客户端代码管理（客户端分区），也可以通过 Redis Cluster 自动实现。在生产环境中，Redis Cluster 是推荐的分区方式，它不仅能自动分配数据，还能提供内置的故障恢复机制。



