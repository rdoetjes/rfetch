# rfetch system info motd
rfetch is simple small motd that can set the following information like motd.
Insead of motd we use the name message.txt

<code>
System name:     <name> 
Kernel version:  <kernel_version>
OS version:      <os_version> 
Number of CPUs:  <cpu_len>
Total Memory:    <total_memory> GB
Load average:    <load.one>, <load.five>, <load.fifteen>
</code>

The message.txt that contains the above listed tags should reside in the ~/.config/rfetch/message.txt directory
and in your .zshrc you just call rfetch
