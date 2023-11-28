# rfetch system info motd
rfetch is simple small motd that can set the following information like motd.
Insead of motd we use the name message.txt

<pre>
Kernel name:     &lt;name&gt; 
Kernel version:  &lt;kernel_version&gt;
OS version:      &lt;os_version&gt; 
Number of CPUs:  &lt;cpu_len&gt;
Total Memory:    &lt;total_memory&gt; GB
Load average:    &lt;load.one&gt;, &lt;load.five&gt;, &lt;load.fifteen&gt;
Primary IP address: &lt;ip&gt;
</pre>

The message.txt that contains the above listed tags should reside in the ~/.config/rfetch/message.txt directory
and in your .zshrc you just call rfetch

This application relies on the sysinfo crate. This crate collects the system data listend above, which inturn is quick and dirtily replaced -- this is efficienter than sed or awk hence this little tool was created.

See message.txt for an example.