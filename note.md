# 1


# 2 最小化内核

## 2.1 引导启动

启动电脑时，主板ROM内存储的固件（firmware）将会运行，步骤如下。

上电自检（power-on self test）-- > 可用内存（available RAM）的检测 --> CPU和其它硬件的预加载
--> 寻找一个可引导的存储介质（bootable disk），引导启动其中的内核（kernel）。

x86架构支持两种固件标准：BIOS（Basic Input/Output System）和UEFI（Unified Extensible Firmware Interface）。

### BIOS启动

电脑启动时，BIOS固件将被加载 --> BIOS固件将会上电自检、初始化硬件，寻找一个可引导的存储介质。

--> 电脑的控制权将被转交给引导程序（bootloader），

--> 引导程序都被切分为一段优先启动、长度不超过512字节、存储在介质开头的第一阶段引导程序（first stage bootloader）

--> 一段随后由其加载的、长度可能较长、存储在其它位置的第二阶段引导程序（second stage bootloader）。


引导程序必须决定内核的位置，并将内核加载到内存。引导程序还需要将CPU从16位的实模式，先切换到32位的保护模式（protected mode），最终切换到64位的长模式（long mode）：此时，所有的64位寄存器和整个主内存（main memory）才能被访问。引导程序的第三个作用，是从BIOS查询特定的信息，并将其传递到内核；如查询和传递内存映射表（memory map）。


